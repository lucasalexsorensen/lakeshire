use super::screenshot::Screenshot;
use anyhow::Result;
use thiserror::Error;

pub struct Scanner {
    screenshot: Screenshot,
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("No control pixel found")]
    NoControlPixelFound,
    #[error("No pixel found")]
    NoPixelFound,
}

impl From<Screenshot> for Scanner {
    fn from(screenshot: Screenshot) -> Self {
        Self { screenshot }
    }
}

impl Scanner {
    /// Scan the bitmap and return the values as a vector of u8
    ///
    /// Procedure:
    /// 1. Verify that the bottom left corner is a control pixel
    /// 2. Determine the size of the cells
    /// 3. Read the meta cell (second cell from the left)
    /// 4. Calculate the grid size from the meta pixel
    /// 5. Place the reader head in the center of the cell 3rd cell from the left
    /// 6. Scan the grid by moving left to right, bottom to top until the expected number of values are read
    /// 7. Return the read values as a vector of u8
    ///
    pub fn scan_bitmap(&mut self) -> Result<Vec<u8>> {
        // let pixels_per_row = self.image_width * 4;
        // let bottom_left_corner_idx = self.image_buffer.len() - pixels_per_row;
        // let pixel = Pixel::from_offset(bottom_left_corner_idx, &self.image_buffer)?;

        // step 1
        let valid_control_pixel = self.validate_bottom_left_control_pixel()?;
        if !valid_control_pixel {
            return Err(ScannerError::NoControlPixelFound.into());
        }

        // step 2
        let cell_size = self.determine_cell_size()?;

        // step 3
        let mut reader = Reader::new(cell_size, self.screenshot.width, self.screenshot.height);

        // skip the fist cell, since it's a control cell
        reader.move_right();
        // then we read the meta pixel. this lets us know how many cells we need to scan afterwards
        let meta_pixel = reader.read_pixel(&self.screenshot.buffer)?;
        let channel_remainder = meta_pixel.r;
        let grid_size = u16::from_le_bytes([meta_pixel.g, meta_pixel.b]);
        let grid_side_length = (grid_size as f32).sqrt().ceil() as u16;
        reader.move_right();

        // now we are ready to scan the rest of the grid
        let mut scanned_values = vec![];
        for i in 2..grid_size {
            let pixel = reader.read_pixel(&self.screenshot.buffer)?;

            // when on the last cell, we need to consider the channel_remainder
            if i == grid_size - 1 {
                let all_channels = [pixel.r, pixel.g, pixel.b];
                let end_idx = match channel_remainder {
                    0 => 3,
                    x => x,
                } as usize;
                let channels_to_add = &all_channels[0..end_idx];
                scanned_values.extend(channels_to_add);
            } else {
                scanned_values.push(pixel.r);
                scanned_values.push(pixel.g);
                scanned_values.push(pixel.b);
            }

            reader.move_right();
            if (i + 1) % grid_side_length == 0 {
                reader.move_next_line();
            }
        }

        Ok(scanned_values)
    }

    fn validate_bottom_left_control_pixel(&self) -> Result<bool> {
        let pixel = Pixel::from_offset(self.bottom_left_corner_idx(), &self.screenshot.buffer)?;
        Ok(pixel.is_control_pixel())
    }

    fn bottom_left_corner_idx(&self) -> usize {
        self.screenshot.buffer.len() - self.screenshot.width * 4
    }

    fn determine_cell_size(&self) -> Result<usize> {
        let mut cell_size = 1;
        loop {
            let pixel = Pixel::from_offset(
                self.bottom_left_corner_idx() + cell_size * 4,
                &self.screenshot.buffer,
            )?;
            if !pixel.is_control_pixel() {
                break Ok(cell_size);
            }

            cell_size += 1;
        }
    }
}

#[derive(Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn from_offset(offset: usize, buffer: &[u8]) -> Result<Self> {
        Ok(Self {
            r: *buffer.get(offset).ok_or(ScannerError::NoPixelFound)?,
            g: *buffer.get(offset + 1).ok_or(ScannerError::NoPixelFound)?,
            b: *buffer.get(offset + 2).ok_or(ScannerError::NoPixelFound)?,
        })
    }

    fn is_control_pixel(&self) -> bool {
        self.r == 199 && self.g == 99
    }
}

/// Represents a reader moving over the bitmap
struct Reader {
    idx: usize,
    stride: usize,
    half_stride: usize,
    image_height: usize,
    image_width: usize,
}

impl Reader {
    pub fn new(stride: usize, image_width: usize, image_height: usize) -> Self {
        let image_buffer_len = image_width * image_height * 4;
        // initial position should be in the center of the bottom left cell
        // we can calculate this using the half stride
        let mut initial_idx = image_buffer_len - image_width * 4;
        let half_stride = (stride as f32 / 2.0).ceil() as usize;
        initial_idx += half_stride * 4;
        initial_idx -= half_stride * image_width * 4;

        Self {
            idx: initial_idx,
            stride,
            half_stride,
            image_width,
            image_height,
        }
    }

    fn read_pixel(&self, buffer: &[u8]) -> Result<Pixel> {
        Pixel::from_offset(self.idx, buffer)
    }

    fn move_right(&mut self) {
        self.idx += self.stride * 4;
    }

    fn move_next_line(&mut self) {
        // reset to the left edge
        self.idx -= self.idx % (self.image_width * 4);
        // move up one line
        self.idx -= self.stride * (self.image_width * 4);
        // move a bit to the right
        self.idx += self.half_stride * 4;
    }
}

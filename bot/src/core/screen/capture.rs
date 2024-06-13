use std::{time::Duration};
use protobuf::{Message};
use scrap::{Capturer, Frame, Display};
use std::io::ErrorKind::WouldBlock;
use crate::core::screen::pixels;
use crate::core::protos::Lakeshire;


pub const ONE_FRAME: Duration = Duration::new(0, 16666667);
pub struct GameGrabber {
    capturer: Capturer,
    width: usize,
    height: usize
}

#[derive(Debug)]
pub enum GrabberError {
    WouldBlock,
    InvalidControlPixel,
    ProtobufDecodeError,
    Other
}

impl GameGrabber {
    pub fn new(display_index: usize) -> GameGrabber {
        let mut result = GameGrabber {
            capturer: GameGrabber::get_capturer(display_index),
            width: 0,
            height: 0
        };
        result.height = result.capturer.height();
        result.width = result.capturer.width();
        return result;
    }

    fn get_capturer (display_index: usize) -> Capturer {
        let mut displays = Display::all().unwrap();
        let display = displays.remove(display_index);
        let capturer = Capturer::new(display).expect("Couldn't begin capture.");
        return capturer;
    }

    fn parse_frame(buffer: &Frame, width: usize) -> Result<Lakeshire::GameState, GrabberError> {
        // Frame is a [u8] of packed BGRA pixels
        // for 1440p, this is:
        // 2560 * 1440 * 4 = 14745600 bytes

        // bottom left pixel is important - save the offset
        let bottom_left_offset = buffer.len() - width*4;

        // Control pixel is the bottom left pixel of the screen
        // we validate it first
        let control_pixel = pixels::ControlPixel::from_offset(&buffer[bottom_left_offset..]);
        if !control_pixel.is_valid() {
            return Err(GrabberError::InvalidControlPixel);
        }

        // Meta pixel is just to the right of the control pixel
        // this contains the msg length
        let meta_pixel = pixels::MetaPixel::from_offset(&buffer[bottom_left_offset + 4*control_pixel.cell_size..]);
        let msg_length = meta_pixel.compute_msg_length();

        let mut pixel_array = Vec::new();
        let square_dim = (msg_length as f32).sqrt().ceil() as u16;
        let mut cursor = bottom_left_offset + 1*4*control_pixel.cell_size;

        for i in 3..(msg_length+1) {
            // advance cursor
            cursor += 4*control_pixel.cell_size;

            // if we are at last pixel, we should only read the remainder
            if i == msg_length {
                let channel_remainder_last_pixel = match meta_pixel.remainder {
                    0 => 3,
                    x => x
                };

                // last pixel
                for i in 0..channel_remainder_last_pixel {
                    pixel_array.push(buffer[cursor+(2-i) as usize]);
                }
                break;
            }

            // read all channels consecutively
            pixel_array.push(buffer[cursor+2]);
            pixel_array.push(buffer[cursor+1]);
            pixel_array.push(buffer[cursor]);

            // if end of row, advance cursor to row above
            if i % square_dim == 0 {
                // jump up a row
                 cursor -= 4*width*control_pixel.cell_size as usize;
                // reset to left
                cursor -= 4*control_pixel.cell_size*(square_dim as usize);
                continue;
            }
        }

        match Message::parse_from_bytes(&pixel_array) {
            Ok(message) => Ok(message),
            Err(error) => {
                println!("Error parsing protobuf: {}", error);
                Err(GrabberError::ProtobufDecodeError)
            }
        }
    }

    pub fn get_frame(&mut self) -> Result<Lakeshire::GameState, GrabberError> {
        let buffer_result = match self.capturer.frame() {
            Ok(buffer) => Ok(buffer),
            Err(error) => {
                if error.kind() == WouldBlock {
                    Err(GrabberError::WouldBlock)
                } else {
                    Err(GrabberError::Other)
                }
            }
        };
        if buffer_result.is_err() {
            return Err(buffer_result.err().unwrap());
        }
        let buffer = buffer_result.unwrap();

        GameGrabber::parse_frame(&buffer, self.width)
    }
}

use anyhow::Result;
#[cfg(feature = "xcap")]
use std::sync::mpsc::Receiver;
use std::{fs::File, thread, time::Duration};
#[cfg(feature = "xcap")]
use xcap::Frame;

#[cfg(feature = "screencapturekit")]
use core_media_rs::cm_sample_buffer::{CMSampleBuffer, error::CMSampleBufferError};

#[cfg(feature = "screencapturekit")]
use screencapturekit::{
    shareable_content::SCShareableContent,
    stream::{
        SCStream, configuration::SCStreamConfiguration, content_filter::SCContentFilter,
        output_trait::SCStreamOutputTrait, output_type::SCStreamOutputType,
    },
};

pub struct Screenshot {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

pub struct ScreenGrabber {
    #[cfg(feature = "fake")]
    fake_state: u64,
    #[cfg(feature = "screencapturekit")]
    stream: SCStream,
    #[cfg(feature = "scrap")]
    capturer: scrap::Capturer,
    #[cfg(feature = "xcap")]
    state: XcapState,
}

#[cfg(feature = "xcap")]
struct XcapState {
    monitor: xcap::Monitor,
}

#[cfg(feature = "scrap")]
impl Default for ScreenGrabber {
    fn default() -> Self {
        let display = scrap::Display::primary().unwrap();
        let capturer = scrap::Capturer::new(display).unwrap();
        Self { capturer }
    }
}

#[cfg(feature = "xcap")]
impl Default for ScreenGrabber {
    fn default() -> Self {
        let mut monitors = xcap::Monitor::all().unwrap();
        let main_monitor = monitors.pop().expect("No monitors found");

        Self {
            state: XcapState {
                monitor: main_monitor,
            },
        }
    }
}

#[cfg(feature = "fake")]
impl Default for ScreenGrabber {
    fn default() -> Self {
        Self { fake_state: 0 }
    }
}

#[cfg(feature = "screencapturekit")]
struct StreamOutput {}

#[cfg(feature = "screencapturekit")]
impl SCStreamOutputTrait for StreamOutput {
    fn did_output_sample_buffer(
        &self,
        sample_buffer: CMSampleBuffer,
        _of_type: SCStreamOutputType,
    ) {
        // println!("sample_buffer: {:?}", sample_buffer);
        let pixel_buffer = loop {
            match sample_buffer.get_pixel_buffer() {
                Ok(pixel_buffer) => break pixel_buffer,
                Err(CMSampleBufferError::CouldNotGetDataBuffer) => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Err(error) => {
                    panic!("error: {:?}", error);
                }
            }
        };
        let width = pixel_buffer.get_width();
        let height = pixel_buffer.get_height();
        println!("width={}, height={}", width, height);
    }
}

#[cfg(feature = "screencapturekit")]
impl Default for ScreenGrabber {
    fn default() -> Self {
        println!("hello world 1");
        let config = SCStreamConfiguration::new();
        println!("hello world 2");
        let display = SCShareableContent::get().unwrap().displays().remove(0);

        let window = SCShareableContent::get()
            .unwrap()
            .windows()
            .into_iter()
            .find(|w| w.title().to_lowercase().contains("world of warcraft"))
            .unwrap();
        println!("hello world 3");
        //let filter = SCContentFilter::new().with_display_including_windows(&display, &[&window]);
        let filter = SCContentFilter::new().with_display_excluding_windows(&display, &[&window]);

        println!("hello world 4");
        let mut stream = SCStream::new(&filter, &config);
        println!("hello world 5");

        stream.add_output_handler(StreamOutput {}, SCStreamOutputType::Screen);

        stream.start_capture().unwrap();
        Self { stream }
    }
}

impl ScreenGrabber {
    #[cfg(feature = "xcap")]
    pub fn get_screenshot(&mut self) -> Result<Screenshot> {
        let monitor_height = self.state.monitor.height()?;
        const SIZE: u32 = 300;
        let image = self
            .state
            .monitor
            .capture_region(0, monitor_height - SIZE, SIZE, SIZE)?;
        let (width, height) = (image.width() as usize, image.height() as usize);
        let buffer = image.into_vec();
        Ok(Screenshot {
            buffer,
            height,
            width,
        })
    }

    #[cfg(feature = "scrap")]
    pub fn get_screenshot(&mut self) -> Result<Screenshot> {
        const ONE_FRAME: Duration = Duration::from_millis(16);

        let (raw_width, raw_height) = (self.capturer.width(), self.capturer.height());
        let raw_buffer = loop {
            let buffer = match self.capturer.frame() {
                Ok(buffer) => buffer.to_vec(),
                Err(error) => {
                    if error.kind() == std::io::ErrorKind::WouldBlock {
                        thread::sleep(ONE_FRAME);
                        continue;
                    } else {
                        return Err(error.into());
                    }
                }
            };
            break buffer;
        };

        let cropped_width = 400;
        let cropped_height = 400;

        // the buffer is made up of rows of raw_width*4 bytes + 64 bytes of padding
        // we want to:
        // 1. crop to the bottom-left region
        // 2. flip BGRA to RGBA
        let mut buffer = vec![0; cropped_width * cropped_height * 4];
        let stride = raw_width * 4 + 64;
        for cropped_row_idx in 0..cropped_height {
            for cropped_col_idx in 0..cropped_width {
                let raw_row_idx = (raw_height - cropped_height) + cropped_row_idx;
                let raw_col_idx = cropped_col_idx;

                let raw_pixel_idx = raw_row_idx * stride + raw_col_idx * 4;
                let b = raw_buffer[raw_pixel_idx];
                let g = raw_buffer[raw_pixel_idx + 1];
                let r = raw_buffer[raw_pixel_idx + 2];
                let a = raw_buffer[raw_pixel_idx + 3];
                let buffer_pixel_idx = cropped_row_idx * cropped_width * 4 + cropped_col_idx * 4;
                buffer[buffer_pixel_idx] = r;
                buffer[buffer_pixel_idx + 1] = g;
                buffer[buffer_pixel_idx + 2] = b;
                buffer[buffer_pixel_idx + 3] = a;
            }
        }

        Ok(Screenshot {
            buffer,
            width: cropped_width,
            height: cropped_height,
        })
    }

    #[cfg(feature = "screencapturekit")]
    pub fn get_screenshot(&mut self) -> Result<Screenshot> {
        Ok(Screenshot {
            buffer: vec![],
            width: 0,
            height: 0,
        })
    }

    #[cfg(feature = "fake")]
    pub fn get_screenshot(&mut self) -> Result<Screenshot> {
        let screenshot1: &[u8] = include_bytes!("../../../../playground/screenshot1.png");
        let screenshot2: &[u8] = include_bytes!("../../../../playground/screenshot2.png");
        let file = if self.fake_state % 2 == 0 {
            screenshot1
        } else {
            screenshot2
        };

        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info()?;
        let mut buf: Vec<u8> = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let bytes = &buf[..info.buffer_size()];
        self.fake_state += 1;
        Ok(Screenshot {
            buffer: bytes.to_vec(),
            width: info.width as usize,
            height: info.height as usize,
        })
    }
}

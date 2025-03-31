use anyhow::Result;
use std::fs::File;

pub struct Screenshot {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

pub struct ScreenGrabber;

impl ScreenGrabber {
    #[cfg(not(feature = "scrap"))]
    pub fn get_frame(&self) -> Result<Screenshot> {
        let monitors = xcap::Monitor::all().unwrap();
        let main_monitor = monitors.first().unwrap();
        let image = main_monitor.capture_image().unwrap();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let buffer = image.into_vec();
        Ok(Screenshot {
            buffer,
            width,
            height,
        })
    }

    #[cfg(feature = "scrap")]
    pub fn get_frame(&self) -> Result<Screenshot> {
        use scrap::{Capturer, Display};
        use std::io::ErrorKind::WouldBlock;
        use std::thread;
        use std::time::Duration;

        const ONE_FRAME: Duration = Duration::from_millis(16);

        let display = Display::primary()?;
        let mut capturer = Capturer::new(display)?;
        let (width, height) = (capturer.width(), capturer.height());

        let buffer = loop {
            let buffer = match capturer.frame() {
                Ok(buffer) => buffer.to_vec(),
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(ONE_FRAME);
                        continue;
                    } else {
                        return Err(error.into());
                    }
                }
            };
            break buffer;
        };
        Ok(Screenshot {
            buffer,
            width,
            height,
        })
    }

    pub fn fake_screenshot(&self) -> Result<Screenshot> {
        let decoder = png::Decoder::new(File::open("../../playground/screenshot2.png")?);
        let mut reader = decoder.read_info()?;
        let mut buf: Vec<u8> = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let bytes = &buf[..info.buffer_size()];
        Ok(Screenshot {
            buffer: bytes.to_vec(),
            width: info.width as usize,
            height: info.height as usize,
        })
    }
}

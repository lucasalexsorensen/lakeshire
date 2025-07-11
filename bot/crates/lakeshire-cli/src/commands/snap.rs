use anyhow::Result;
use lakeshire_core::screenshot::ScreenGrabber;
use std::{fs::File, io::BufWriter};

pub fn snap_command(output: Option<String>) -> Result<()> {
    let mut screen_grabber = ScreenGrabber::default();
    let screenshot = screen_grabber.get_screenshot()?;

    let width = screenshot.width as u32;
    let height = screenshot.height as u32;
    let buffer = screenshot.buffer;
    println!("width: {}, height: {}", width, height);

    let output_path = output.unwrap_or("screenshot.png".to_string());
    let file = File::create(&output_path).unwrap();
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buffer).unwrap();

    println!("Image saved to `{}`", output_path);
    Ok(())
}

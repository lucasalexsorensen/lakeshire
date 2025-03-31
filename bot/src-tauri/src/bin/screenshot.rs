use anyhow::Result;
use bot_lib::core::screenshot::ScreenGrabber;
use repng::Options;
use std::fs::File;

fn main() -> Result<()> {
    let screen_grabber = ScreenGrabber;
    let screenshot = screen_grabber.get_frame()?;

    let w = screenshot.width;
    let h = screenshot.height;
    let buffer = screenshot.buffer;

    // Save the image.

    repng::encode(
        File::create("screenshot.png").unwrap(),
        w as u32,
        h as u32,
        &buffer,
    )
    .unwrap();

    println!("Image saved to `screenshot.png`.");
    Ok(())
}

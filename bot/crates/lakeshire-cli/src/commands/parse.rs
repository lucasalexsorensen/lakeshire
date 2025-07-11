use anyhow::Result;
use lakeshire_core::serialization::deserialize_game_state;
use lakeshire_core::{scanner::Scanner, screenshot::ScreenGrabber};

pub fn parse_command() -> Result<()> {
    let mut grabber = ScreenGrabber::default();
    let screenshot = grabber.get_screenshot()?;
    let mut scanner = Scanner::from(screenshot);
    let game_state_bytes = scanner.scan_bitmap()?;
    let game_state = deserialize_game_state(&game_state_bytes)?;
    println!("Game state: {:?}", game_state);
    Ok(())
}

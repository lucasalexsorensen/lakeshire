use anyhow::Result;
use bot_lib::core::scanner::Scanner;
use bot_lib::core::screenshot::{ScreenGrabber, Screenshot};
use bot_lib::core::serialization::deserialize_game_state;
use std::thread;
use std::time::Duration;

const ONE_SECOND: Duration = Duration::from_secs(1);
fn main() -> Result<()> {
    let screen_grabber = ScreenGrabber;

    loop {
        let t0 = std::time::Instant::now();
        let screenshot = screen_grabber.get_frame()?;
        // let screenshot = screen_grabber.fake_screenshot()?;
        // println!("capture took {:?}", t0.elapsed());

        let mut scanner = Scanner::from(screenshot);
        let result = scanner.scan_bitmap();

        if result.is_err() {
            println!("error: {:?}", result.err().unwrap());
            thread::sleep(ONE_SECOND);
            continue;
        }
        let unwrapped = result.unwrap();
        let result_slice = unwrapped.as_slice();
        let game_state = deserialize_game_state(result_slice)?;
        println!("game_state: {:?}", game_state);

        thread::sleep(ONE_SECOND);
    }
}

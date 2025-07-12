use std::{sync::atomic::Ordering, time::Duration};
use tokio::time::sleep;

use crate::{bus::EVENT_BUS, event::BusEvent, state::RUNNING_SCAN_THREAD};

pub fn scanner_task() {
    #[cfg(feature = "fake")]
    let mut faker = faker::GameStateFaker::default();
    #[cfg(not(feature = "fake"))]
    let mut screen_grabber = lakeshire_core::screenshot::ScreenGrabber::default();
    let bus = EVENT_BUS.clone();
    loop {
        if !RUNNING_SCAN_THREAD.load(Ordering::Relaxed) {
            // sleep(Duration::from_millis(500)).await;
            std::thread::sleep(Duration::from_millis(500));
            continue;
        }

        let t0 = std::time::Instant::now();
        #[cfg(feature = "fake")]
        let game_state_bytes = {
            std::thread::sleep(Duration::from_millis(20));
            faker.generate()
        };
        #[cfg(not(feature = "fake"))]
        let game_state_bytes = {
            let screenshot = screen_grabber
                .get_screenshot()
                .expect("Failed to get screenshot");
            let scan_result = lakeshire_core::scanner::Scanner::from(screenshot)
                .scan_bitmap()
                .ok();

            if let Some(scan_result) = scan_result {
                scan_result
            } else {
                // sleep(Duration::from_millis(500)).await;
                std::thread::sleep(Duration::from_millis(500));
                continue;
            }
        };

        let duration = t0.elapsed();
        println!("Time taken: {:?}", duration);
        bus.publish(BusEvent::GameStateScanned {
            bytes: game_state_bytes.clone(),
        });
    }
}

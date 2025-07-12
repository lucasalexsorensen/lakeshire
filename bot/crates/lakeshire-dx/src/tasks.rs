use crate::global::GlobalState;
use dioxus::prelude::*;
use lakeshire_core::{
    scanner::Scanner, screenshot::ScreenGrabber, serialization::deserialize_game_state,
};

pub fn start_scanner_task(mut global_state: GlobalState) {
    std::thread::spawn(move || {
        #[cfg(feature = "fake")]
        let mut faker = faker::GameStateFaker::default();
        #[cfg(not(feature = "fake"))]
        let mut grabber = ScreenGrabber::default();
        loop {
            #[cfg(not(feature = "fake"))]
            {
                let screenshot = grabber.get_screenshot().expect("Failed to get screenshot");
                let mut scanner = Scanner::from(screenshot);
                let game_state_bytes = scanner.scan_bitmap().expect("Failed to scan bitmap");
                if let Ok(new_game_state) = deserialize_game_state(&game_state_bytes) {
                    global_state.game_state.set(Some(new_game_state));
                } else {
                    println!("Failed to deserialize game state!");
                    println!("game_state_bytes: {:?}", &game_state_bytes);
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
            #[cfg(feature = "fake")]
            {
                let new_game_state = faker.generate();
                global_state.game_state.set(Some(new_game_state));
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }
    });
}

pub fn start_active_window_monitor_task(mut active_window_title: SyncSignal<Option<String>>) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(250));
        let old_val = active_window_title();
        let window =
            active_win_pos_rs::get_active_window().expect("Error while getting active window");

        let new_val = Some(window.title);

        if new_val != old_val {
            println!("new_val: {:?}", &new_val);
            active_window_title.set(new_val);
        }
    });
}

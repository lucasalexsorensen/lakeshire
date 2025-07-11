use anyhow::Result;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use lakeshire_core::serialization::deserialize_game_state;
use lakeshire_core::{scanner::Scanner, screenshot::ScreenGrabber};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub fn face_command() -> Result<()> {
    let mut grabber = ScreenGrabber::default();

    // facing goes from 0 to 2*PI
    // 0 = north, pi/2 = west, pi = south, 3pi/2 = east

    // the goal is to face north
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    const LEFT_KEY: Key = Key::Unicode('æ');
    const RIGHT_KEY: Key = Key::Unicode('ø');

    const ROTATION_RADS_PER_MS: f32 = std::f32::consts::PI / 1000.0;
    const INTER_FRAME_DELAY_MS: f32 = 40.0;
    const INPUT_DELAY_MS: f32 = 10.0;
    const MICROSTEP_THRESHOLD_RADS: f32 =
        ROTATION_RADS_PER_MS * (INTER_FRAME_DELAY_MS + INPUT_DELAY_MS);

    // sleep for a bit so we have a chance to tab into the game
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let scan_loop_should_stop = Arc::new(AtomicBool::new(false));
    let angle_error = Arc::new(Mutex::new(None::<f32>));

    let angle_error_clone = angle_error.clone();
    let scan_loop_should_stop_clone = scan_loop_should_stop.clone();
    let scan_loop_thread = std::thread::spawn(move || {
        while !scan_loop_should_stop_clone.load(Ordering::Relaxed) {
            let screenshot = grabber.get_screenshot().expect("Failed to get screenshot");
            let mut scanner = Scanner::from(screenshot);
            let game_state_bytes = scanner.scan_bitmap().expect("Failed to scan bitmap");
            let game_state = deserialize_game_state(&game_state_bytes)
                .expect("Failed to deserialize game state");
            let facing = game_state.player.pos_info.facing as f32 / 1e10;
            // figure out how far we are from north. this delta should be between -pi and pi
            let normalized = facing.sin().atan2(facing.cos());
            let angle_error = 0.0f32 - normalized;
            {
                println!("setting angle error");
                *angle_error_clone.lock().unwrap() = Some(angle_error);
            }
        }
    });

    let angle_error_clone = angle_error.clone();
    loop {
        println!("reading angle error");
        let angle_error = { angle_error_clone.lock().unwrap() };
        if angle_error.is_none() {
            std::thread::sleep(std::time::Duration::from_millis(15));
            continue;
        }
        let angle_error = angle_error.unwrap();
        let abs_diff = angle_error.abs();
        println!("angle_error: {angle_error}, abs_diff: {abs_diff}");
        let key = if angle_error.is_sign_negative() {
            RIGHT_KEY
        } else {
            LEFT_KEY
        };
        if abs_diff < 0.02 {
            break;
        } else if abs_diff < (2.0 * MICROSTEP_THRESHOLD_RADS) {
            println!("microstepping");
            enigo.key(key, Direction::Press).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(3));
            enigo.key(key, Direction::Release).unwrap();
        } else {
            // we know rotation speed, so we can calculate the time to hold the key
            enigo.key(key, Direction::Press).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(30));
            enigo.key(key, Direction::Release).unwrap();
        }
    }

    scan_loop_should_stop.store(true, Ordering::Relaxed);
    scan_loop_thread
        .join()
        .expect("Failed to join scan loop thread");

    Ok(())
}

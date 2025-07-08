use anyhow::Result;
use lakeshire_core::scanner::serialization::{FixedPositionInfo, deserialize_game_state};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs::File, io::BufWriter};

use clap::{Parser, Subcommand};
use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use lakeshire_core::scanner::{scanner::Scanner, screenshot::ScreenGrabber};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Bench {
        #[arg(short = 'n', long, default_value_t = 100)]
        count: u32,
    },
    Snap {
        output: Option<String>,
    },
    Parse,
    Face,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Bench { count } => bench_command(count),
        Command::Parse => parse_command(),
        Command::Snap { output } => snap_command(output),
        Command::Face => face_command(),
    }
}

fn bench_command(count: u32) -> Result<()> {
    let mut grabber = ScreenGrabber::default();

    println!("Benchmarking {} screenshots", count);
    let durations: Result<Vec<Duration>> = (0..count)
        .map(|_| {
            let t0 = std::time::Instant::now();
            let _ = grabber.get_screenshot()?;
            let t1 = std::time::Instant::now();
            let duration = t1.duration_since(t0);
            Ok(duration)
        })
        .collect();
    let avg_duration = durations?.iter().sum::<Duration>() / count;
    println!("Average time taken:: {:?}", avg_duration);

    Ok(())
}

fn parse_command() -> Result<()> {
    let mut grabber = ScreenGrabber::default();
    let screenshot = grabber.get_screenshot()?;
    let mut scanner = Scanner::from(screenshot);
    let game_state_bytes = scanner.scan_bitmap()?;
    let game_state = deserialize_game_state(&game_state_bytes)?;
    println!("Game state: {:?}", game_state);
    Ok(())
}

fn snap_command(output: Option<String>) -> Result<()> {
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

fn face_command() -> Result<()> {
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

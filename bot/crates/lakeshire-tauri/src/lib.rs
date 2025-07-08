use std::sync::Mutex;
use std::time::Duration;

use tauri::{ipc::Channel, ipc::Response, State};
use tauri::{AppHandle, Emitter, Listener, Manager};
use tauri_plugin_log::{Target, TargetKind};

#[derive(Default)]
struct AppState {}

#[tauri::command]
fn init_scan_loop(app: AppHandle, on_event: Channel<Vec<u8>>) -> bool {
    std::thread::spawn(move || {
        let mut screen_grabber = lakeshire_core::scanner::screenshot::ScreenGrabber::default();
        loop {
            let t0 = std::time::Instant::now();
            let screenshot = {
                screen_grabber
                    .get_screenshot()
                    .map_err(|e| e.to_string())
                    .unwrap()
            };
            let game_state_bytes =
                lakeshire_core::scanner::scanner::Scanner::from(screenshot).scan_bitmap();
            if let Err(e) = game_state_bytes {
                println!("Error scanning game state: {}", e);
                continue;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    });
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let logger = tauri_plugin_log::Builder::new()
        .clear_targets()
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Webview,
        ))
        .build();

    tauri::Builder::default()
        .plugin(logger)
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init_scan_loop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

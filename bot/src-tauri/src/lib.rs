use tauri::ipc::Response;

pub mod core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_game_state() -> Result<Response, String> {
    let screen_grabber = core::screenshot::ScreenGrabber;
    let screenshot = screen_grabber.get_frame().map_err(|e| e.to_string())?;
    // let screenshot = screen_grabber
    //     .fake_screenshot()
    //     .map_err(|e| e.to_string())?;
    let mut scanner = core::scanner::Scanner::from(screenshot);
    let result = scanner.scan_bitmap().map_err(|e| e.to_string())?;
    Ok(Response::new(result))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, read_game_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

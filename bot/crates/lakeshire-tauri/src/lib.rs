mod event;
mod faker;

use anyhow::Result;
use event::{BotStateEvent, GameStateScannedEvent};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{App, AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutEvent, ShortcutState};

#[derive(Default)]
struct AppState {}

static RUNNING_SCAN_THREAD: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn start_scanner(app: AppHandle) {
    RUNNING_SCAN_THREAD.store(true, Ordering::Relaxed);
    app.emit_to(
        "main",
        "bot-state-changed",
        BotStateEvent::ScannerRunningChanged {
            scanner_running: RUNNING_SCAN_THREAD.load(Ordering::Relaxed),
        },
    )
    .expect("Failed to emit bot-state-changed");
}

#[tauri::command]
fn stop_scanner(app: AppHandle) {
    RUNNING_SCAN_THREAD.store(false, Ordering::Relaxed);
    app.emit_to(
        "main",
        "bot-state-changed",
        BotStateEvent::ScannerRunningChanged {
            scanner_running: RUNNING_SCAN_THREAD.load(Ordering::Relaxed),
        },
    )
    .expect("Failed to emit bot-state-changed");
}

fn start_scanner_thread(app: AppHandle) -> bool {
    std::thread::spawn(move || -> Result<()> {
        let mut screen_grabber = lakeshire_core::screenshot::ScreenGrabber::default();
        let mut faker = faker::GameStateFaker::default();
        loop {
            if !RUNNING_SCAN_THREAD.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(500));
                continue;
            }

            // let screenshot = screen_grabber.get_screenshot()?;
            // let game_state_bytes =
            //     lakeshire_core::scanner::Scanner::from(screenshot).scan_bitmap()?;
            let game_state_bytes = faker.generate();
            std::thread::sleep(Duration::from_millis(20));

            app.emit_to(
                "main",
                "game-state-scanned",
                GameStateScannedEvent {
                    bytes: game_state_bytes,
                },
            )?;
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
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));

            setup_window(app)?;
            #[cfg(desktop)]
            setup_shortcuts(app)?;

            start_scanner_thread(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_scanner, stop_scanner])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_window(app: &mut App) -> Result<()> {
    let window = app
        .get_webview_window("main")
        .ok_or(anyhow::anyhow!("Failed to get window"))?;
    window.set_always_on_top(true)?;
    Ok(())
}

fn setup_shortcuts(app: &mut App) -> Result<()> {
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts(["ctrl+l", "ctrl+i"])?
            .with_handler(|app, shortcut, event| {
                handle_shortcut(app, shortcut, event).expect("Failed to handle shortcut");
            })
            .build(),
    )?;
    Ok(())
}

fn handle_shortcut(app: &AppHandle, shortcut: &Shortcut, event: ShortcutEvent) -> Result<()> {
    if event.state != ShortcutState::Pressed {
        return Ok(());
    }

    if shortcut.matches(Modifiers::CONTROL, Code::KeyI) {
        println!("Toggle scanner");
        RUNNING_SCAN_THREAD.fetch_xor(true, Ordering::Relaxed);
        app.emit_to(
            "main",
            "bot-state-changed",
            BotStateEvent::ScannerRunningChanged {
                scanner_running: RUNNING_SCAN_THREAD.load(Ordering::Relaxed),
            },
        )?;
    } else if shortcut.matches(Modifiers::CONTROL, Code::KeyL) {
        let windows = app.webview_windows();
        windows.values().try_for_each(|window| -> Result<()> {
            if window.is_visible()? {
                window.hide()?;
            } else {
                window.show()?;
            }
            Ok(())
        })?;
    } else {
        println!("Unknown shortcut: {:?}", shortcut);
    }
    Ok(())
}

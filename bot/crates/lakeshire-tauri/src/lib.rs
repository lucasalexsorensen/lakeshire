mod bus;
mod commands;
mod event;
mod faker;
mod shortcuts;
mod state;
mod tasks;

use anyhow::Result;
use std::sync::Mutex;
use tauri::{App, Manager, Runtime};

use crate::state::AppState;

#[allow(dead_code)]
enum LoggingStrategy {
    Console,
    CrabNebula,
    Webview,
}

const LOGGING_STRATEGY: LoggingStrategy = LoggingStrategy::Console;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    builder = register_logging(builder);
    builder = register_plugins(builder);

    builder = builder.setup(|app| {
        app.manage(AppState::default());

        setup_window(app)?;
        shortcuts::setup_shortcuts(app)?;
        spawn_tasks(app)?;

        Ok(())
    });

    builder = builder.invoke_handler(tauri::generate_handler![
        commands::initialize_app,
        commands::uninitialize_app,
    ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_window(app: &mut App) -> Result<()> {
    let window = app
        .get_webview_window("main")
        .ok_or(anyhow::anyhow!("Failed to get window"))?;
    window.set_always_on_top(true)?;
    // window.set_decorations(false)?;
    // window.set_title_bar_style(tauri::TitleBarStyle::Overlay)?;s
    Ok(())
}

fn register_plugins<R: Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder.plugin(tauri_plugin_opener::init())
}

fn register_logging<R: Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    match LOGGING_STRATEGY {
        LoggingStrategy::Console => {
            std::env::set_var("RUST_LOG", "info");
            tracing_subscriber::fmt::init();
            builder
        }
        LoggingStrategy::CrabNebula => builder.plugin(tauri_plugin_devtools::init()),
        LoggingStrategy::Webview => {
            let logger = tauri_plugin_log::Builder::new()
                .clear_targets()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build();
            builder.plugin(logger)
        }
    }
}

fn spawn_tasks(app: &mut App) -> Result<()> {
    std::thread::spawn(move || {
        tasks::scanner_task();
    });
    // tauri::async_runtime::spawn(tasks::scanner_task());
    tauri::async_runtime::spawn(tasks::control_task());
    tauri::async_runtime::spawn(tasks::active_window_task());
    tauri::async_runtime::spawn(tasks::event_emitter_task(app.handle().clone()));
    Ok(())
}

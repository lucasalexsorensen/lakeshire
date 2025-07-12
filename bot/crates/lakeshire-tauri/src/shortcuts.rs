use std::sync::atomic::Ordering;

use anyhow::Result;
use tauri::{App, AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutEvent, ShortcutState};

use crate::{bus::EVENT_BUS, event::BusEvent, state::RUNNING_SCAN_THREAD};

pub fn setup_shortcuts(app: &mut App) -> Result<()> {
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
        toggle_scanner();
    } else if shortcut.matches(Modifiers::CONTROL, Code::KeyL) {
        toggle_window(app)?;
    } else {
        println!("Unknown shortcut: {:?}", shortcut);
    }
    Ok(())
}

fn toggle_scanner() {
    let bus = EVENT_BUS.clone();
    RUNNING_SCAN_THREAD.fetch_xor(true, Ordering::Relaxed);
    bus.publish(BusEvent::ScannerStateChanged {
        value: RUNNING_SCAN_THREAD.load(Ordering::Relaxed),
    });
}

fn toggle_window(app: &AppHandle) -> Result<()> {
    let windows = app.webview_windows();
    windows.values().try_for_each(|window| -> Result<()> {
        if window.is_visible()? {
            window.hide()?;
        } else {
            window.show()?;
        }
        Ok(())
    })
}

use anyhow::Result;
use lakeshire_core::serialization::deserialize_game_state;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tokio::time::sleep;
use tracing::{error, info};

use crate::{bus::EVENT_BUS, event::BusEvent};

pub async fn control_task() {
    let bus = EVENT_BUS.clone();

    let mut receiver = bus.subscribe();
    loop {
        let event = receiver.recv().await.expect("Failed to receive event");
        process_event(event).unwrap_or_else(|e| error!("Failed to process event: {}", e));
    }
}

fn process_event(event: BusEvent) -> Result<()> {
    match event {
        BusEvent::GameStateScanned {
            bytes: game_state_bytes,
        } => process_game_state_bytes(&game_state_bytes)?,
        BusEvent::ActiveWindowChanged {
            title: window_title,
        } => {
            // if window_title.contains("Lakeshire") {
            //     info!("Lakeshire is active");
            // } else {
            //     info!("Lakeshire is not active");
            // }
        }
        _ => {}
    }
    Ok(())
}

fn process_game_state_bytes(game_state_bytes: &[u8]) -> Result<()> {
    let game_state = deserialize_game_state(game_state_bytes)?;
    Ok(())
}

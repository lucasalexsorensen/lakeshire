use tauri::{AppHandle, Emitter};

use crate::bus::EVENT_BUS;

pub async fn event_emitter_task(app: AppHandle) {
    let bus = EVENT_BUS.clone();
    loop {
        let event = bus
            .subscribe()
            .recv()
            .await
            .expect("Failed to receive event");
        app.emit("bus-event", event).expect("Failed to emit event");
    }
}

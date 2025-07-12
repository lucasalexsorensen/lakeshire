use crate::{bus::EVENT_BUS, event::BusEvent};

#[tauri::command]
pub fn initialize_app() {
    start_scanner();
}

#[tauri::command]
pub fn uninitialize_app() {
    stop_scanner();
}

pub fn start_scanner() {
    let bus = EVENT_BUS.clone();
    bus.publish(BusEvent::ScannerStateChanged { value: true });
}

pub fn stop_scanner() {
    let bus = EVENT_BUS.clone();
    bus.publish(BusEvent::ScannerStateChanged { value: false });
}

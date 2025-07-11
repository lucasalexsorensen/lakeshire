use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum BotStateEvent {
    ScannerRunningChanged { scanner_running: bool },
}

#[derive(Clone, Serialize)]
pub struct GameStateScannedEvent {
    pub bytes: Vec<u8>,
}

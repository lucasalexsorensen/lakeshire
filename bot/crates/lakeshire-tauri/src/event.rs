use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum BusEvent {
    ActiveWindowChanged { title: String },
    GameStateScanned { bytes: Vec<u8> },
    ScannerStateChanged { value: bool },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_with_active_window_changed() {
        let event = BusEvent::ActiveWindowChanged {
            title: "test".to_string(),
        };
        let serialized = serde_json::to_string(&event).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"ActiveWindowChanged","title":"test"}"#
        );
    }

    #[test]
    fn test_serialize_with_game_state_scanned() {
        let event = BusEvent::GameStateScanned {
            bytes: vec![1, 2, 3],
        };
        let serialized = serde_json::to_string(&event).unwrap();
        assert_eq!(serialized, r#"{"type":"GameStateScanned","bytes":[1,2,3]}"#);
    }

    #[test]
    fn test_serialize_with_scanner_state_changed() {
        let event = BusEvent::ScannerStateChanged { value: true };
        let serialized = serde_json::to_string(&event).unwrap();
        assert_eq!(serialized, r#"{"type":"ScannerStateChanged","value":true}"#);
    }
}

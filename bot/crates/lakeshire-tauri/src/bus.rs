use once_cell::sync::Lazy;
use tokio::sync::broadcast;

use crate::event::BusEvent;

pub static EVENT_BUS: Lazy<EventBus> = Lazy::new(EventBus::new);

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<BusEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(128);
        EventBus { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<BusEvent> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: BusEvent) {
        self.sender.send(event).expect("Failed to publish event");
    }
}

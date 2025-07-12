use active_win_pos_rs::get_active_window;
use std::time::Duration;
use tokio::time::sleep;

use crate::{bus::EVENT_BUS, event::BusEvent, state::CURRENT_WINDOW_TITLE};

pub async fn active_window_task() {
    let bus = EVENT_BUS.clone();
    loop {
        let active_window = get_active_window().ok();

        if let Some(active_window) = active_window {
            *CURRENT_WINDOW_TITLE.lock().unwrap() = Some(active_window.title.clone());
            bus.publish(BusEvent::ActiveWindowChanged {
                title: active_window.title,
            });
        }

        sleep(Duration::from_millis(200)).await;
    }
}

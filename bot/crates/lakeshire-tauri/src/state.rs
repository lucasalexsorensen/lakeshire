use once_cell::sync::Lazy;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub static RUNNING_SCAN_THREAD: AtomicBool = AtomicBool::new(false);
pub static CURRENT_WINDOW_TITLE: Lazy<Arc<Mutex<Option<String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[derive(Clone, Default)]
pub struct AppState {}

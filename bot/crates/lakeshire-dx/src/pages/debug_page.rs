use crate::{components::Plot, global::GlobalState};
use dioxus::prelude::*;
use lakeshire_core::serialization::protos::GameState;
use lazy_static::lazy_static;
use std::collections::VecDeque;

lazy_static! {
    pub static ref X_VALUES: Vec<f64> = (0..100).map(|x| x as f64).collect();
}

#[component]
pub fn DebugPage() -> Element {
    let global_state = use_context::<GlobalState>();
    let mut recent_values = use_signal(|| VecDeque::from(vec![0.0; 100]));

    let game_state = global_state.game_state.read();
    if game_state.is_none() {
        return rsx! { "Loading..." };
    }
    let game_state = game_state.clone().unwrap();

    let recent_values = &mut recent_values.write();
    recent_values.pop_front();
    recent_values.push_back(game_state.player.pos_info.facing as f64 / 1e10);

    let values = X_VALUES
        .iter()
        .zip(recent_values.iter())
        .map(|(x, y)| (*x, *y))
        .collect();
    rsx! {
        Plot { values }
    }
}

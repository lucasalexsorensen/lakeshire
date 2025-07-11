use dioxus::prelude::*;
use lakeshire_core::serialization::protos::GameState;

#[derive(Clone, Copy)]
pub struct GlobalState {
    pub game_state: SyncSignal<Option<GameState>>,
}

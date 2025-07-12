use crate::components::AreaMap;
use crate::global::GlobalState;
use dioxus::prelude::*;
use lakeshire_core::serialization::FixedPositionInfo;

#[component]
pub fn HomePage() -> Element {
    let global_state = use_context::<GlobalState>();
    let game_state = global_state.game_state.read();
    if game_state.is_none() {
        return rsx! { "Loading..." };
    }

    let game_state = game_state.clone().unwrap();

    let area_id = match game_state.player.pos_info.map_id {
        1411 => 14,
        _ => panic!("Unknown area id: {}", game_state.player.pos_info.map_id),
    };
    let pos_info = FixedPositionInfo::from(game_state.player.pos_info);

    rsx! {
        AreaMap { area_id, pos_info }
    }
}

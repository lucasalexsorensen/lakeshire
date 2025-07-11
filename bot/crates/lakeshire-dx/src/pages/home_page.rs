use crate::components::AreaMap;
use crate::global::GlobalState;
use dioxus::prelude::*;

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
    let player_pos = (
        game_state.player.pos_info.map_x as f32 / 1e14,
        game_state.player.pos_info.map_y as f32 / 1e14,
    );
    let player_facing = game_state.player.pos_info.facing as f32 / 1e10;

    rsx! {
        AreaMap { area_id, player_pos, player_facing }
    }
}

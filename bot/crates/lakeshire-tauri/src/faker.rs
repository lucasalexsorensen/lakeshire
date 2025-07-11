use lakeshire_core::serialization::protos::GameState;
use prost::Message;

#[derive(Default)]
pub struct GameStateFaker {
    i: usize,
}

impl GameStateFaker {
    pub fn generate(&mut self) -> Vec<u8> {
        self.i += 1;

        let mut state = GameState::default();
        state.player.pos_info.map_id = 1411;

        // rotation should go from [0, 2pi]
        let rotation = (self.i as f32 * 0.25) % (2.0 * std::f32::consts::PI);
        state.player.pos_info.facing = (rotation * 1e10) as u64;

        state.player.pos_info.map_x = (0.5 * 1e14) as u64;
        state.player.pos_info.map_y = (0.5 * 1e14) as u64;

        Message::encode_to_vec(&state)
    }
}

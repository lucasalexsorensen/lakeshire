use lakeshire_core::serialization::protos::GameState;

#[derive(Default)]
pub struct GameStateFaker {
    i: usize,
}

impl GameStateFaker {
    pub fn generate(&mut self) -> GameState {
        self.i += 1;

        let mut state = GameState::default();
        state.player.pos_info.map_id = 1411;

        // rotation should go from [0, 2pi]
        let rotation = (self.i as f32 * 0.25) % (2.0 * std::f32::consts::PI);
        state.player.pos_info.facing = (rotation * 1e10) as u64;

        let new_pos = (
            (self.i as f32 / 25.0).sin() * 0.05 + 0.5,
            (self.i as f32 / 25.0).cos() * 0.05 + 0.5,
        );
        state.player.pos_info.map_x = (new_pos.0 * 1e14) as u64;
        state.player.pos_info.map_y = (new_pos.1 * 1e14) as u64;

        state
    }
}

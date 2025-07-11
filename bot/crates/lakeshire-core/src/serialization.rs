use anyhow::Result;
use prost::Message;

pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/lakeshire.rs"));
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedPositionInfo {
    pub map_id: u32,
    pub map_position: Point,
    pub facing: f64,
    pub world_position: Point,
}

impl From<protos::Position> for FixedPositionInfo {
    fn from(pos_info: protos::Position) -> Self {
        let raw_facing = pos_info.facing as f64 / 1e10;

        // first, adjust for the constant -pi/2 offset
        let adjusted_facing = raw_facing + std::f64::consts::PI / 2.0;
        // then, normalize to the range [-pi, pi]
        let mut normalized_facing = ((adjusted_facing + std::f64::consts::PI)
            % (2.0 * std::f64::consts::PI))
            - std::f64::consts::PI;
        // Handle the edge case for -π
        if normalized_facing == -std::f64::consts::PI {
            normalized_facing = std::f64::consts::PI;
        }

        Self {
            map_id: pos_info.map_id as u32,
            map_position: Point {
                x: pos_info.map_x as f64 / 1e14,
                y: pos_info.map_y as f64 / 1e14,
            },
            facing: normalized_facing,
            world_position: Point {
                x: pos_info.world_x as f64 / 1e5,
                y: pos_info.world_y as f64 / 1e5,
            },
        }
    }
}

pub fn deserialize_game_state(data: &[u8]) -> Result<protos::GameState> {
    let game_state = protos::GameState::decode(data)?;
    Ok(game_state)
}

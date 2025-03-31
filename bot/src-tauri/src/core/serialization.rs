use anyhow::Result;
use prost::Message;

pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/lakeshire.rs"));
}

pub fn deserialize_game_state(data: &[u8]) -> Result<protos::GameState> {
    println!("Deserializing data: {:?}", data);
    let game_state = protos::GameState::decode(data)?;
    Ok(game_state)
}

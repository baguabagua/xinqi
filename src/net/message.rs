use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Message {
    CreateNewGame { game_name: String, player_order: bool },
    AcceptCreateNewGame { game_name: String, player_order: bool },
    Step(String),
}

#[derive(Event)]
pub struct ReceiveRemoteStep {
    pub step: String,
}

#[derive(Event)]
pub struct SendRemoteStep {
    pub step: String,
}
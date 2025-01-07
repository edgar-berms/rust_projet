use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct ServerState {
    pub teams: HashMap<String, String>,  // Nom équipe -> Code d'accès
    pub players: HashMap<String, String>, // Nom joueur -> Nom équipe
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    RegisterTeam { name: String },
    RegisterPlayer { access_code: String, player_name: String },
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    TeamRegistered { access_code: String, players_needed: u32 },
    PlayerRegistered { message: String },
    Error { message: String },
}

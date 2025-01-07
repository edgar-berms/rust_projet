use crate::state::ServerState;
use rand::random;
use std::collections::HashMap;

pub fn register_team(team_name: String, state: &mut ServerState) -> String {
    // Générer un code d'accès unique pour l'équipe
    let access_code = format!("{:X}", random::<u32>());
    state.teams.insert(team_name.clone(), access_code.clone());
    access_code
}

pub fn register_player(player_name: String, access_code: String, state: &mut ServerState) -> Result<String, String> {
    if state.teams.values().any(|code| *code == access_code) {
        state.players.insert(player_name.clone(), access_code);
        Ok(player_name)
    } else {
        Err("Invalid access code".into())
    }
}

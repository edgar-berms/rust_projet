use warp::Reply;
use crate::state::{ClientMessage, ServerMessage};
use crate::state::ServerState;
use crate::utils::register_player;
use std::sync::Arc;
use std::sync::Mutex;

pub async fn handle_register_team(
    msg: ClientMessage,
    state: Arc<Mutex<ServerState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let ClientMessage::RegisterTeam { name } = msg {
        let mut state = state.lock().unwrap();
        let access_code = crate::utils::register_team(name, &mut state);
        Ok(warp::reply::json(&ServerMessage::TeamRegistered {
            access_code,
            players_needed: 4, // Par exemple, 4 joueurs pour une équipe
        }))
    } else {
        Ok(warp::reply::json(&ServerMessage::Error {
            message: "Invalid message".into(),
        }))
    }
}

pub async fn handle_register_player(
    msg: ClientMessage,
    state: Arc<Mutex<ServerState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let ClientMessage::RegisterPlayer { access_code, player_name } = msg {
        let mut state = state.lock().unwrap();
        match register_player(player_name, access_code, &mut state) {
            Ok(player_name) => Ok(warp::reply::json(&ServerMessage::PlayerRegistered {
                message: format!("Player {} registered successfully", player_name),
            })),
            Err(err) => Ok(warp::reply::json(&ServerMessage::Error {
                message: err,
            })),
        }
    } else {
        Ok(warp::reply::json(&ServerMessage::Error {
            message: "Invalid message".into(),
        }))
    }
}

mod state;
mod handlers;
mod utils;

use warp::Filter;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use crate::state::ServerState;
use crate::handlers::{handle_register_team, handle_register_player};
use crate::utils::register_team;
use common::maze::Labyrinthe;

#[tokio::main]
async fn main() {
    // Initialisation de l'état du serveur (avec une Mutex pour la gestion concurrente)
    let state = Arc::new(Mutex::new(ServerState::default()));

    // Routes du serveur
    let register_team_route = warp::post()
        .and(warp::path("register_team"))
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_register_team);

    let register_player_route = warp::post()
        .and(warp::path("register_player"))
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_register_player);

    let routes = register_team_route.or(register_player_route);

    // Démarrer le serveur sur l'adresse 127.0.0.1:8778
    warp::serve(routes).run(([127, 0, 0, 1], 8778)).await;
}

// Fonction utilitaire pour partager l'état du serveur
fn with_state(
    state: Arc<Mutex<ServerState>>,
) -> impl warp::Filter<Extract = (Arc<Mutex<ServerState>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

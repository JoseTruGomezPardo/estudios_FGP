use crate::Controllers; // <--- 'crate' busca la carpeta Controllers desde la raíz de src
use axum::{routing::get, Router};
use std::sync::Arc;
use sqlx::MySqlPool;
use tower_http::cors::{Any, CorsLayer};

pub fn request(estado: Arc<MySqlPool>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)       // Permite peticiones desde cualquier origen (Tauri lo necesita)
        .allow_methods(Any)      // Permite GET, POST, PUT, etc.
        .allow_headers(Any);     // Permite cualquier cabecera (como Content-Type)
    Router::new()
        .route("/usuarios", get(Controllers::get_usuarios_controller::obtener_usuarios))
        .layer(cors)
        .with_state(estado)
}
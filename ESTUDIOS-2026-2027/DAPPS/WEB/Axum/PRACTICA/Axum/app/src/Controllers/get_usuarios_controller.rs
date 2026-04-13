use axum::{extract::State, Json, http::StatusCode};
use std::sync::Arc;
use sqlx::MySqlPool;
use crate::Models::usuario_model::Usuario; // Importamos el modelo

pub async fn obtener_usuarios(
    State(pool): State<Arc<MySqlPool>>
) -> Result<Json<Vec<Usuario>>, StatusCode> {
    
    // El controlador delega la responsabilidad al modelo
    match Usuario::buscar_todos(&pool).await {
        Ok(usuarios) => Ok(Json(usuarios)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
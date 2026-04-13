use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use crate::Data::conf::config::Config;

pub async fn conectar() -> MySqlPool {
    let config = Config::from_env();
    
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("No se pudo conectar a la base de datos MySQL")
}
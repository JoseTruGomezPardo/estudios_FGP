use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Carga el archivo .env
        let url = env::var("DATABASE_URL").expect("DATABASE_URL no configurada en .env");
        Self { database_url: url }
    }
}
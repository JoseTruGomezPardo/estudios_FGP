use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

#[derive(Serialize, Deserialize, sqlx::FromRow)] 
pub struct Usuario {
    pub id: i32,             // En MySQL 'int' suele mapear a i32
    pub nombre: Option<String>,    // Usamos Option porque en tu DB dice "NULL"
    pub apellidos: Option<String>, // Usamos Option porque en tu DB dice "NULL"
    pub edad: Option<i32>,         // Usamos Option porque en tu DB dice "NULL"
}

impl Usuario {
    pub async fn buscar_todos(pool: &MySqlPool) -> Result<Vec<Usuario>, sqlx::Error> {
        // Asegúrate de pedir las columnas que existen
        let usuarios = sqlx::query_as::<_, Usuario>("SELECT id, nombre, apellidos, edad FROM usuarios")
            .fetch_all(pool)
            .await?;
        
        Ok(usuarios)
    }
}
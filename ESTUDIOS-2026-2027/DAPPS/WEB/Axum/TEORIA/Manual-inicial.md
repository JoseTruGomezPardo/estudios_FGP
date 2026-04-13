## 1. INTRODUCCIÓN:

Elegimos Axum, porque es un framework más potente que laravel y más compatible con Tauri.

1. Rendimiento y Concurrencia Extremos

Esta es la ventaja más evidente. Mientras que Laravel es un framework interpretado (PHP), Axum es compilado y corre sobre Tokio, el runtime asíncrono más potente de Rust.

    Rendimiento bruto: Axum puede manejar más de 20,000 peticiones por segundo en hardware modesto, mientras que Laravel suele quedarse en los cientos (o pocos miles con optimizaciones agresivas como Octane).

    Consumo de memoria: Un binario de Axum puede iniciarse consumiendo apenas 10-15 MB de RAM. Laravel, al levantar todo su ecosistema y el motor PHP, consume órdenes de magnitud más.

    Latencia: Las respuestas en Axum suelen medirse en microsegundos o milisegundos bajos, ideales para sistemas de trading, juegos en tiempo real o microservicios de alta carga.

2. Seguridad de Tipos y "Si compila, funciona"

Laravel depende mucho de la naturaleza dinámica de PHP y de la "magia" (fachadas, inyección de dependencias mágica). Axum aprovecha el sistema de tipos de Rust:

    Extractores Tipados: En Axum, si necesitas un JSON o un parámetro en la URL, lo defines en los argumentos de la función. Si el cliente envía algo que no coincide con el tipo, el framework lo rechaza automáticamente antes de que se ejecute tu lógica.

    Prevención de errores en producción: Olvídate de los clásicos Call to a member function on null de PHP. El compilador de Rust te obliga a manejar cada posible error y estado nulo antes de que el código llegue a producción.

3. Despliegue Minimalista (Binarios Estáticos)

    Laravel: Necesitas instalar PHP, gestionar extensiones (bcmath, gd, xml), configurar un servidor web (Nginx/Apache) y un gestor de procesos (PHP-FPM).

    Axum: Compilas tu proyecto y obtienes un único archivo binario. Lo copias a tu servidor o lo metes en una imagen de Docker minúscula (usando scratch o alpine) y listo. No necesita dependencias externas en el sistema operativo.

4. Mantenimiento a Largo Plazo y Refactorización

En Laravel, cambiar el nombre de una columna o un modelo puede ser una pesadilla si tienes una base de código grande (puedes romper cosas que el IDE no detecta).

    En Axum (usando herramientas como SQLx), tus consultas SQL se validan en tiempo de compilación. Si cambias un tipo en la base de datos y no actualizas el código, el proyecto simplemente no compilará. Esto hace que las refactorizaciones sean extremadamente seguras.


## 2. AJUSTES GENERALES:

- archivo Cargo.toml:

```

*************Contiene los paquetes necesarios*****************
[package]
name = "app"
version = "0.1.0"
edition = "2024"

*************Aqui es donde tenemos las dependencias necesarias***************
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] } # Fundamental para JSON
sqlx = { version = "0.8", features = ["runtime-tokio", "mysql", "macros"] }
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

```

- archivo .env:

```

*************Es el archivo de la conexión a la bbdd***************

DATABASE_URL=mysql://root:root@localhost:32768/axum

```


- archivo main.rs

```

*************Es el archivo principal***************

mod Data;
mod Controllers;
mod Models;
mod Router; // <--- Importa la carpeta Router

use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 1. Inicializa el sistema de logs
    // Esto permite que veamos por consola qué está pasando en el servidor.
    tracing_subscriber::fmt()
        .with_env_filter("app=debug,axum=info") // Define el nivel de detalle
        .init();
    // 2. Llama a tu función personalizada para abrir la conexión con la DB (MySQL/SQLite).
    // Usamos .await porque conectar a una base de datos toma tiempo y no queremos bloquear el programa.
    let pool = Data::database::conectar().await;
    // 3. Envolvemos la conexión en un "Arc" (Atomic Reference Counted).
    // Esto permite que la conexión se comparta de forma segura entre todos los hilos 
    // del servidor; así, cada petición que llegue puede usar la misma base de datos.
    let compartido = Arc::new(pool);

    // 4. Llamamos a tu función de enrutamiento pasando la conexión (el estado).
    // Aquí es donde se configuran qué URLs (como /usuarios) van a qué controladores.
    // 'app' ahora contiene toda la lógica de rutas de tu servidor.
    let app = Router::request(compartido);

    // 5. Creamos un "escuchador" (listener) de red.
    // Le decimos al sistema operativo que reserve el puerto 3000 en la IP local (127.0.0.1)
    // para que nuestra aplicación pueda recibir tráfico ahí.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // 6. Un simple mensaje en consola para que sepas que todo ha arrancado correctamente.
    println!("🚀 Servidor corriendo en http://localhost:3000");
    
    // 7. Ponemos el servidor en marcha.
    // Esta función se queda "escuchando" indefinidamente, pasando cada conexión 
    // que llega a nuestro sistema de rutas ('app').
    axum::serve(listener, app).await.unwrap();
}

```

- archivo router.rs

```

*********************Es el archivo de enrrutamiento********************************

use crate::Controllers; // <--- 'crate' busca la carpeta Controllers desde la raíz de src
use axum::{routing::get, Router};
use std::sync::Arc;
use sqlx::MySqlPool;

pub fn request(estado: Arc<MySqlPool>) -> Router {
    Router::new()
        .route("/usuarios", get(Controllers::get_usuarios_controller::obtener_usuarios))
        .with_state(estado)
}

```

- archivos mod.rs

```
pub mod router;

// Re-exportamos la función 'request' para que sea accesible
pub use router::request;

*********** Son aquellos archivos, los cuales sirven para poder exportar los demás archivos que contiene toda la app. **********

```

## 3. Arquitectura MVC:

#### 3.1. Database y migraciones:

##### A. Database

```

************** Son los archivos de configuracion y conexion de la bbdd ****************

************** config.rs ****************

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


************** database.rs ****************

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

```

##### B. Migraciones

1. Instalar la herramienta de SQLx (CLI)

Para gestionar las migraciones, necesitas una pequeña utilidad en tu terminal. Abre tu consola y ejecuta:
Bash

```
cargo install sqlx-cli --no-default-features --features mysql

```

(Esto instalará el comando sqlx preparado específicamente para MySQL).

2. Preparar el proyecto

Antes de crear la primera migración, asegúrate de que tu archivo .env tiene la URL correcta y que la base de datos existe (aunque esté vacía). Luego, ejecuta este comando para que SQLx se prepare:
Bash

```
sqlx database setup

```

(Esto crea la base de datos si no existe y una tabla especial llamada _sqlx_migrations que sirve para llevar el control de qué cambios ya se han aplicado).
3. Crear tu primera migración

Ahora vamos a crear los "planos" para tu tabla de usuarios. Ejecuta:
Bash

```

sqlx migrate add usuarios

```

Esto creará una carpeta llamada migrations en la raíz de tu proyecto y dentro verás un archivo con un nombre parecido a este: 202310271030_crear_tabla_usuarios.sql.
4. Escribir el SQL

Abre ese archivo recién creado y escribe el código SQL para crear tu tabla. Este es el contenido que debería tener:
SQL

```

-- migrations/[timestamp]_crear_tabla_usuarios.sql

CREATE TABLE IF NOT EXISTS usuarios (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(100),
    apellidos VARCHAR(100),
    edad INT,
    creado_en TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

```


#### 3.2. Models:

```

************** Son los archivos que contemplan la logica de las tablas de la bbdd, la cual se va a utilizar luego en los Controladores. ****************


************************* Tabla de usuario ***************************

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




```


#### 3.3. Controllers:

```

************** Son los archivos que contemplan la logica de toda la app y se ponen en contacto con la Vista. ****************

************************* Un ejemplo de ello es: ***************************

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



```

#### 3.4. El Middleware de CORS:

```

************************* Configuración de CORS ***************************
// Define una política de seguridad que permite la comunicación entre el 
// frontend (Tauri/Navegador) y este backend. Sin esto, el navegador 
// bloquearía las peticiones por seguridad al venir de puertos distintos.

let cors = CorsLayer::new()
        .allow_origin(Any)       // Acepta peticiones desde cualquier origen
        .allow_methods(Any)      // Permite todos los verbos HTTP (GET, POST, etc.)
        .allow_headers(Any);     // Permite enviar cualquier cabecera en la petición
***************************************************************************


```
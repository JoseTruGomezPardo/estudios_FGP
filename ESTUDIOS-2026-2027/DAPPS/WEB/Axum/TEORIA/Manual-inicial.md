## 1. INTRODUCCIÓN:

Elejimos Axum, porque es un framework más potente que laravel y más compatible con Tauri.

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
    // 1. Llama a tu función personalizada para abrir la conexión con la DB (MySQL/SQLite).
    // Usamos .await porque conectar a una base de datos toma tiempo y no queremos bloquear el programa.
    let pool = Data::database::conectar().await;
    // 2. Envolvemos la conexión en un "Arc" (Atomic Reference Counted).
    // Esto permite que la conexión se comparta de forma segura entre todos los hilos 
    // del servidor; así, cada petición que llegue puede usar la misma base de datos.
    let compartido = Arc::new(pool);

    // 3. Llamamos a tu función de enrutamiento pasando la conexión (el estado).
    // Aquí es donde se configuran qué URLs (como /usuarios) van a qué controladores.
    // 'app' ahora contiene toda la lógica de rutas de tu servidor.
    let app = Router::request(compartido);

    // 4. Creamos un "escuchador" (listener) de red.
    // Le decimos al sistema operativo que reserve el puerto 3000 en la IP local (127.0.0.1)
    // para que nuestra aplicación pueda recibir tráfico ahí.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // 5. Un simple mensaje en consola para que sepas que todo ha arrancado correctamente.
    println!("🚀 Servidor corriendo en http://localhost:3000");
    
    // 6. Ponemos el servidor en marcha.
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

#### 3.1. Data:

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

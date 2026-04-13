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
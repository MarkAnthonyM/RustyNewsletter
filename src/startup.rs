use super::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;
use std::sync::Arc;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // Wrap db connection in an Arc for clonability
    let connection = Arc::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register pointer copy to db connection as part of the application state
            .data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

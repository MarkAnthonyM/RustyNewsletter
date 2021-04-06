use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Redirect all "log's" events to our subscriber
    // Setup logger to print all logs at info-level or higher if
    // RUST_LOG environment variable has not been set
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);
    
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;

    Ok(())
}

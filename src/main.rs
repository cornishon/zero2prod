use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use zero2prod::{
    configuration::get_configuration,
    startup,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", config.application.host, config.application.port);
    tracing::info!("starting application on {}", address);

    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(config.database.connection_string().expose_secret())
        .expect("Failed to create connection pool");

    startup::run(listener, connection_pool).await
}

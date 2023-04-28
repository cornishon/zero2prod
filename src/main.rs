use secrecy::ExposeSecret;
use sqlx::PgPool;
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

    tracing::info!("hello from main");
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let connection_pool = PgPool::connect_lazy(config.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");

    startup::run(listener, connection_pool).await
}

use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    startup::run(listener).await
}

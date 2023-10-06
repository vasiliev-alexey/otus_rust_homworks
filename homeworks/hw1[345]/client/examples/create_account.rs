use client::BankClient;
use log::{error, info};

use shared::constants::{LOG_LEVEL, SERVER_PATH};

fn main() {
    // Initialize the logger based on the environment variable `LOG_LEVEL`.
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));

    // Connect to the bank server.
    let client = BankClient::connect(SERVER_PATH);

    // Check if there was an error connecting to the server.
    if let Err(err) = client {
        error!("Failed to connect: {}", err);
        return;
    } else {
        info!("Successfully connected to the bank server");
    }

    // Unwrap the client from the `Result`.
    let mut client = client.unwrap();

    // Create an account with the name "Hello".
    let result = client.create_account("Hello");

    // Handle any errors that occurred during account creation.
    if let Err(err) = result {
        error!("Failed to create account: {}", err);
    }
}

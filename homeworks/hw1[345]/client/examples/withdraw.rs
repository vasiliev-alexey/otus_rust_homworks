use client::client::BankClient;
use log::{error, info};
use std::error::Error;

use shared::constants::{LOG_LEVEL, SERVER_ADDRESS};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger based on the environment variable `LOG_LEVEL`.
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(LOG_LEVEL));

    // Connect to the bank server.
    let mut client = BankClient::connect(SERVER_ADDRESS)?;

    info!("Successfully connected to the bank server");

    // Create an account with the name "Hello".
    client.create_account("Hello")?;

    // Deposit 100.0 into the account with the name "Hello".
    client.deposit("Hello", 100.0)?;

    // Withdraw 50.0 from the account with the name "Hello".
    client.withdraw("Hello", 50.0)?;

    // Withdraw 80.0 from the account with the name "Hello".
    let result = client.withdraw("Hello", 80.0);

    if let Err(err) = result {
        error!("Withdraw error: {:?}", err);
    }

    Ok(())
}

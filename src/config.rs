//! Environment Config Entrypoint
//!
//! Create a singular entrypoint for environment
//! configuration mechanisms to reduce config
//! code smell trying to fetch env vars all
//! over the place!
use std::env;

/// Configuration
///
/// Struct to group, contain, and manage configuration
/// taken from either environment file or doppler
/// environment injections
pub struct Config {
    pub rpc_url: String,
}

impl Config {
    pub fn new() -> Self {
        // ensure that environment variables are loaded
        ensure_load_env();
        Self {
            rpc_url: try_from_env("RPC_URL"),
        }
    }
}

/// Loads the environment variables from the .env file
///
/// This is the default
pub fn ensure_load_env() {
    println!("ENV loaded from .env");
    // ok to unwrap, if it fails, we should know
    dotenvy::dotenv().unwrap();
}

/// Loads specific variable from environment
///
/// Panics when expected variable is missing
pub fn try_from_env(name: &str) -> String {
    let err = format!("Expected {name} in the environment");
    env::var(name).expect(&err)
}

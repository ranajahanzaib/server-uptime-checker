use serde::Deserialize;
use std::fs;

/// Represents a server configuration with its URL and HTTP method.
#[derive(Deserialize)]
pub struct Server {
    pub url: String,    // The URL of the server.
    pub method: String, // The HTTP method (e.g., GET, POST).
}

/// Represents the overall configuration, including servers and the check interval.
#[derive(Deserialize)]
#[allow(dead_code)] // Allow unused fields for potential future expansion.
pub struct Config {
    pub servers: Vec<Server>, // A list of servers to check.
    pub check_interval: u64,  // The interval (in seconds) for performing checks.
}

/// Loads the configuration from a JSON file.
///
/// # Arguments
/// * `path` - The file path to the JSON configuration file.
///
/// # Returns
/// * A `Config` struct populated with the data from the file.
///
/// # Panics
/// * If the file cannot be read or the JSON format is invalid.
pub fn load_config(path: &str) -> Config {
    let data = fs::read_to_string(path).expect("Failed to read config file");
    serde_json::from_str(&data).expect("Invalid config format")
}

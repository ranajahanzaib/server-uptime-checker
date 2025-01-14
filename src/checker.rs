use reqwest;
use std::net::TcpStream;

/**
 * Checks if a URL is reachable via an HTTP GET request.
 *
 * # Arguments
 * * `url` - The URL to check.
 *
 * # Returns
 * * `true` if the response status is 2xx, otherwise `false`.
 */
pub async fn check_http(url: &str) -> bool {
    reqwest::get(url)
        .await
        .map(|res| res.status().is_success())
        .unwrap_or(false)
}

/**
 * Verifies if a TCP connection can be established to the specified address.
 *
 * # Arguments
 * * `ip` - The target IP address and port (e.g., "192.168.1.1:80").
 *
 * # Returns
 * * `true` if the connection succeeds, otherwise `false`.
 */
pub fn check_ping(ip: &str) -> bool {
    TcpStream::connect(ip).is_ok()
}

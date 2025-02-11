use reqwest::{Client, Error};
use tracing::{error, trace, warn};
use std::time::Duration;

const IP_API_ENDPOINT: &str = "https://iproxy.containerscrew.com/";

// Function to configure the reqwest client, optionally using a proxy
fn configure_client() -> Result<Client, Error> {
    let client_builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(10)); // Set timeout of 10 seconds

    client_builder.build()
}

// Main function to perform the request, retrying if the request fails
pub async fn get_geolocation(ip: &str) {
    // Configure the client with or without proxy
    let client = match configure_client() {
        Ok(c) => c,
        Err(e) => {
            error!("Error configuring client: {:?}", e);
            return;
        }
    };

    // Make the request
    let response = client
        .get(format!("{}{}", IP_API_ENDPOINT, ip))
        .send()
        .await;

    match response {
        Ok(resp) => {
            // If the request is successful
            if resp.status().is_success() {
                trace!("Request succeeded with status: {}", resp.status());
            } else {
                warn!(
                    "Request failed with status: {})",
                    resp.status(),
                );
            }
        }
        Err(e) => {
            error!(
                "Request error: {:?}",
                e
            );
        }
    }

}

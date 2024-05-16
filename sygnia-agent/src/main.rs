extern crate reqwest;

use std::time::Duration;

use serde_json::json;

/***********************************************************************************************************************
In this exercise you will implement the business logic of a simple Rust agent.

The agent's main task is to periodically collect as much data as it can from the host it's running on and send it to
the server as part of a `heartbeat` request. This request should be sent every 15 minutes to ensure up-to-date
information.

Your task is to implement as many collectors as you can (in the given time), from the operating system you're working on.
Ideas for collectors:
* Disk usage statistics
* Memory usage statistics
* Network interfaces list
* Disk volumes list
(Or anything else you may come up with)

A few tips before you go:
1. The server is configured to log the body of the requests it receives. Make sure that the data sent by your agent
appears as expected in these logs.
2. Feel free to install and import any external packages that may assist you in implementing the collectors.
3. Aim to implement as many collectors as possible within the given time constraints.
***********************************************************************************************************************/

// Specify the URL to which you want to send the heartbeat request
const SERVER_URL: &str = "http://127.0.0.1:5000/heartbeat";
const HEARTBEAT_INTERVAL: u64 = 15;

fn main() {
    // Infinite loop
    loop {

        // TODO: Your code goes here

        // Send heartbeat request
        let _ = send_heartbeat_request(SERVER_URL);

        // Wait for 15 minutes before sending the next heartbeat
        std::thread::sleep(Duration::from_secs(HEARTBEAT_INTERVAL));
    }
}

// Function to send heartbeat request
fn send_heartbeat_request(url: &str) -> Result<(), reqwest::Error> {
    // Create a new synchronous client
    let client = reqwest::blocking::Client::new();

    let json_data = json!({
        "heartbeat_message": "hello!",
    });

    // Send a GET request to the specified URL
    let response = client.post(url).
        header("Content-Type", "application/json").
        json(&json_data).
        send()?;

    match response.error_for_status() {
        Ok(_res) => Ok(()),
        Err(err) => {
            eprintln!("Error sending heartbeat: {}", err);
            Err(err)
        }
    }
}

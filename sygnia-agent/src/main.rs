extern crate reqwest;

use std::time::Duration;

use serde_json::json;

// In this exercise you will implement the business logic of a simple Rust agent.
//
// The agent's main task is to periodically collect as much data as it can from the host it's
// running on and send it to the server as part of a `heartbeat` request (defined to be sent every
// 15 minutes)
//
// Your task is to implement as many collectors as you can (in the given time). Ideas for
// collectors:
// * Disk usage statistics
// * Memory usage statistics
// * Network interfaces list
// * Disk volumes list
// (Or anything else you think that's relevant)
//
// Few tips before you go -
// * The server is congifured to log the body of the requests it gets. Make sure it looks like
// you'd expect.
// * Feel free to install and import any external package that might help you.
// * Try to implement as many as possible.


// Specify the URL to which you want to send the heartbeat request
const SERVER_URL: &str = "http://127.0.0.1:5000/heartbeat";

fn main() {
    // Infinite loop
    loop {

        // TODO: Your code goes here

        // Send heartbeat request
        let _ = send_heartbeat_request(SERVER_URL);

        // Wait for 15 minutes before sending the next heartbeat
        std::thread::sleep(Duration::from_secs(15));
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


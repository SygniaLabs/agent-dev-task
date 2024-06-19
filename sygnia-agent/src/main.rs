extern crate reqwest;

use std::time::Duration;

use serde_json::json;

/***********************************************************************************************************************
In this exercise you will implement the business logic of a simple Rust agent.
We will focus on core capabilities of the agent, on the operating system you're working on.

Step 1:
    One of the agent's main tasks is to periodically collect as much data as it can from the
    host it's running on and send it to the server as part of a `heartbeat` request.
    This request should be sent every 15 seconds to ensure up-to-date information.

    In this step, choose 2-3 general information collectors and implement them.
    Ideas for collectors:
    * Disk usage statistics
    * Memory usage statistics
    * Network interfaces list
    * Disk volumes list
    (Or anything else you may come up with)

Step 2:
    Another core capability the agent must have is running commands from its server.
    In this step, we will implement a "protocol" between the server and the agent.
    See sygnia-cnc-server/server.py:11 and choose a proper way the server can ask the agent
    for a "dirlist" command.
    Then, implement the actual command execution on the agent's side - get the command from the
    server, execute it and save the result to a local file. Assume that the "dirlist" command
    starts its walking from the host's root directory (or C:\ if it's a Windows machine), and
    choose the format you want to represent the result in.

A few tips before you go:
1. The server is configured to log the body of the requests it receives.
   Make sure that the data sent by your agent appears as expected in these logs.
   Feel free to add any helpful debug-prints you need.
2. Feel free to install and import any external packages that may assist you in implementing the
   collectors or the dirlist resolver.

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

use discord_rpc_client::Client as DiscordClient;
use playerctl_rust_wrapper::Playerctl;
use regex::Regex;
use std::{thread, time};

fn main() {
    // Compile the regex pattern
    let re = Regex::new(r"music.youtube.com").unwrap();

    // Create new discord client
    let mut drpc = DiscordClient::new(1327589296177418291);

    // Add ready event
    drpc.on_ready(|_| {
        println!("Ready");
    });

    // Start the client
    drpc.start();

    loop {
        // Get track metadata
        let metadata = Playerctl::metadata().unwrap();

        // Check if track comes from Youtube Music
        if re.is_match(&metadata.url) {
            if let Err(err) = drpc.set_activity(|a| {
                a.state(format!("by {}", metadata.artist))
                    .details(format!("{} [{}]", metadata.title, metadata.album))
            }) {
                println!("Failed to set presence: {}", err);
            }
        }

        // Wait 5 seconds
        thread::sleep(time::Duration::from_secs(5));
    }
}

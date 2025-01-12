use discord_rpc_client::Client as DiscordClient;
use playerctl_rust_wrapper::Playerctl;
use regex::Regex;
use std::{thread, time};

fn main() {
    // Compile the regex pattern
    let re = Regex::new(r"music.youtube.com").unwrap();

    // Create new discord client
    let mut drpc = DiscordClient::new(1327589296177418291);

    // Whether the program has ownership of current activity
    let mut has_activity = false;

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
        if !metadata.url.is_empty() && re.is_match(&metadata.url) {
            if !has_activity {
                has_activity = true;

                let mut title = metadata.title.clone();

                if !metadata.album.is_empty() {
                    title.push_str(format!(" | {}", metadata.album).as_str());
                }

                if let Err(err) =
                    drpc.set_activity(|a| a.state(format!("by {}", metadata.artist)).details(title))
                {
                    eprintln!("Failed to set presence: {}", err);
                }
            }
        } else if has_activity {
            has_activity = false;
            drpc.clear_activity().unwrap();
        }

        // Wait 3 seconds
        thread::sleep(time::Duration::from_secs(3));
    }
}

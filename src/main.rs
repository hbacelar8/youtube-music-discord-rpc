use discord_rpc_client::Client as DiscordClient;
use playerctl_rust_wrapper::{Playerctl, TrackMetadata};
use regex::Regex;
use std::{thread, time};

const REFRESH_RATE_S: u64 = 2;

#[derive(Default)]
struct ActivityData {
    track_metadata: TrackMetadata,
}

fn main() {
    // Compile the regex pattern
    let re = Regex::new(r"music.youtube.com").unwrap();

    // Activity struct data
    let mut activity_data = ActivityData::default();
    let mut has_activity = false;

    // Create new discord client
    let mut drpc = DiscordClient::new(1327589296177418291);

    // Add ready event
    drpc.on_ready(|_| {
        println!("Discord connection ready");
    });

    // Start the client
    drpc.start();

    loop {
        if let Ok(metadata) = Playerctl::metadata() {
            if !metadata.url.is_empty() && re.is_match(&metadata.url) {
                if metadata.title != activity_data.track_metadata.title {
                    has_activity = true;
                    activity_data.track_metadata = metadata;

                    let mut title = activity_data.track_metadata.title.clone();

                    if !activity_data.track_metadata.album.is_empty() {
                        title.push_str(
                            format!(" | {}", activity_data.track_metadata.album).as_str(),
                        );
                    }

                    if let Err(err) = drpc.set_activity(|a| {
                        a.state(format!("by {}", activity_data.track_metadata.artist))
                            .details(title)
                    }) {
                        eprintln!("Failed to set presence: {}", err);
                    } else {
                        println!(
                            "Presence set => Title: {} | Album: {} | Artist: {}",
                            activity_data.track_metadata.title,
                            activity_data.track_metadata.album,
                            activity_data.track_metadata.artist
                        );
                    }
                }
            } else if has_activity {
                println!("Player no longer detected, clearing activity");
                activity_data = ActivityData::default();
                has_activity = false;
                drpc.clear_activity().unwrap();
            }
        }

        // Wait 3 seconds
        thread::sleep(time::Duration::from_secs(REFRESH_RATE_S));
    }
}

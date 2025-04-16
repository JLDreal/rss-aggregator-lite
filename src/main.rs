use core::{RssController, SettingsController};
use std::{error::Error, fs::{create_dir, File}, io::Write, path::PathBuf};

use regex::Regex;
use rss::Channel;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut settings = SettingsController::new();
    settings.load().await.unwrap();
    settings
        .rss_controller
        .channels
        .iter()
        .for_each(|x| println!("title: {}", x.title));
    
}
async fn download_content(mut channel: Channel) -> Result<Channel, Box<dyn Error>> {
    // Create a directory for the channel
    let channel_dir = PathBuf::from(&channel.title);
    create_dir(&channel_dir)?;

    let regex = Regex::new(r#"<img[^>]+src="([^">]+)""#)?;

    for item in &mut channel.items {
        // Ensure item.content is a String
        let content = item.content.as_mut().unwrap();

        // Find all captures in the item content
        let images = regex.captures_iter(content).collect::<Vec<_>>();

        for capture in images {
            // Get the URL from the capture group
            if let Some(url) = capture.get(1) {
                let url = url.as_str();

                // Download the content
                let content = reqwest::get(url).await?.bytes().await?;

                // Create a file to save the content
                let file_name = "test";
                let file_path = channel_dir.join(&file_name);
                let mut file = File::create(&file_path)?;
                file.write_all(&content)?;

                
            }
        }
    }

    Ok(channel) // Return the channel after processing
}

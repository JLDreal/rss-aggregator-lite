use infer::Infer;
use regex::Regex;
use rss::Channel;
use serde_derive::Deserialize;
use std::fs::{self, File, create_dir};
use std::path::PathBuf;
use std::{error::Error, io::BufReader, io::Write};
use toml;

pub struct RssController {
    pub channels: Vec<Channel>,
    pub feed_urls: Vec<String>,
    pub online: bool,
}

impl RssController {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            feed_urls: Vec::new(),
            online: false,
        }
    }

    pub fn add_feed(&mut self, url: String) {
        if !self.feed_urls.contains(&url) {
            self.feed_urls.push(url);
        }
    }

    pub async fn get_feeds(&mut self) {
        self.channels = Vec::new();
        for i in 0..self.feed_urls.len() {
            self.get_feed(i).await;
        }
    }

    pub async fn get_feed(&mut self, index: usize) {
        let mut tmp_channels: Vec<Channel> = Vec::new();
        if self.online {
            match self
                .load_feed(
                    &self
                        .get_file_name(self.feed_urls.get(index).unwrap())
                        .as_str(),
                )
                .await
            {
                Ok(res) => self.channels.push(res),
                Err(_) => println!("[!] No local version."),
            };
        } else {
            match self.download_feed(self.feed_urls.get(index).unwrap()).await {
                Ok(res) => self.channels.push(res),
                Err(_) => match self
                    .load_feed(
                        &self
                            .get_file_name(self.feed_urls.get(index).unwrap())
                            .as_str(),
                    )
                    .await
                {
                    Ok(res) => tmp_channels.push(res),
                    Err(_) => println!("[!] No internet and no local version."),
                },
            };
        }
    }

    fn get_file_name(&self, url: &str) -> String {
        let regex = Regex::new(r"\/\/(.*\..*)\/").unwrap();
        regex
            .captures(url)
            .expect("file wrong idk.")
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace(".", "_")
            .replace("/", "")
            + ".xml"
    }
    fn get_image_name(&self, url: &str, filetype: &str) -> String {
        let regex = Regex::new(r"//.*\..*/([[:alnum:]_-]*)\.*").unwrap();
        let mut reg = regex
            .captures(url)
            .expect("no url base")
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace("/", "");

        format!("{}.{}", reg, filetype)
    }

    async fn download_feed(&self, url: &str) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::get(url) //
            .await?
            .bytes()
            .await?;
        let channel = Channel::read_from(&content[..])?;
        let channel = self.download_content(channel).await.unwrap();

        let mut file = File::create(self.get_file_name(url)).expect("Unable to create file");
        channel.write_to(file)?;

        Ok(channel)
    }

    async fn download_content(&self, mut channel: Channel) -> Result<Channel, Box<dyn Error>> {
        // Create a directory for the channel
        let channel_dir = PathBuf::from(&channel.title);
        let _ = create_dir(&channel_dir).is_err();
        let infer = Infer::new();

        let regex = Regex::new(r#"<img[^>]+src="([^">]+)""#)?;

        for item in &mut channel.items {
            // Ensure item.content is a String
            let content = match item.content.clone() {
                Some(res) => res,
                None => break,
            };

            // Find all captures in the item content
            let images = regex.captures_iter(&content).collect::<Vec<_>>();

            for capture in images {
                // Get the URL from the capture group
                if let Some(url) = capture.get(1) {
                    let url = url.as_str();

                    // Download the content
                    let image_data = reqwest::get(url).await?.bytes().await?;

                    // getting file type
                    let extension = infer
                        .get(&image_data)
                        .map(|kind| kind.extension())
                        .unwrap_or("jpg");

                    // Create a file to save the content
                    let file_name = self.get_image_name(url, extension);
                    let file_path = channel_dir.join(&file_name);
                    let mut file = File::create(&file_path)?;
                    file.write_all(&image_data)?;
                    println!("{}", url);
                    println!("{:?}", file_path);

                    item.content = Some(
                        item.content
                            .as_mut()
                            .unwrap()
                            .replace(url, file_path.to_str().unwrap())
                            .to_string(),
                    );
                }
            }
        }

        Ok(channel) // Return the channel after processing
    }

    async fn load_feed(&self, filename: &str) -> Result<Channel, Box<dyn Error>> {
        let file = File::open(filename)?;
        let channel = Channel::read_from(BufReader::new(file))?;
        Ok(channel)
    }

    pub fn remove_feed(&mut self, url: &str) {
        if let Some(index) = self.feed_urls.iter().position(|x| x == url) {
            self.channels.remove(index);
            self.feed_urls.remove(index);
        }
    }

    pub fn get_channels(&self) -> Vec<&Channel> {
        self.channels.iter().collect()
    }
}

#[derive(Deserialize)]
struct Settings {
    is_offline: bool,
    download_behavior: String,
}

#[derive(Deserialize)]
struct Feed {
    feeds: Vec<String>,
}

#[derive(Deserialize)]
struct Data {
    pub settings: Settings,
    pub feed: Feed,
}

pub struct SettingsController {
    pub rss_controller: RssController,
    pub settings: Settings,
}

impl SettingsController {
    pub fn new() -> Self {
        Self {
            rss_controller: RssController::new(),
            settings: Settings {
                is_offline: false,
                download_behavior: "".to_string(),
            },
        }
    }
    pub async fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string("settings/settings.toml")?;
        let data: Data = toml::from_str(&content)?;

        self.settings = data.settings;
        self.rss_controller.feed_urls = data.feed.feeds;
        self.rss_controller.online = self.settings.is_offline.clone();
        self.rss_controller.get_feeds().await;

        Ok(())
    }
}

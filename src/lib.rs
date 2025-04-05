use regex::Regex;
use rss::Channel;
use serde_derive::Deserialize;
use std::fs::{self, File};
use std::iter;
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
        if self.online {
            match self
                .load_feed(
                    format!(
                        "{}.xml",
                        &self.get_file_name(self.feed_urls.get(index).unwrap())
                    )
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
                        format!(
                            "{}.xml",
                            &self.get_file_name(self.feed_urls.get(index).unwrap())
                        )
                        .as_str(),
                    )
                    .await
                {
                    Ok(res) => self.channels.push(res),
                    Err(_) => println!("[!] No internet and no local version."),
                },
            }
        }
    }

    fn get_file_name(&self, url: &str) -> String {
        let regex = Regex::new(r"\/\/(.*\..*)\/").unwrap();
        regex
            .captures(url)
            .expect("no url base")
            .get(0)
            .unwrap()
            .as_str()
            .to_string()
            .replace(".", "_")
            .replace("/", "")
    }

    async fn download_feed(&self, url: &str) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::get(url) //
            .await?
            .bytes()
            .await?;
        let channel = Channel::read_from(&content[..])?;
        let mut file = File::create(format!("{}.xml", self.get_file_name(url)))
            .expect("Unable to create file");
        file.write(&content)?;

        Ok(channel)
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

use regex::Regex;
use rss::{Channel, Item};
use serde_derive::Deserialize;
use std::fs::{self, File, create_dir};
use std::iter;
use std::path::{Path, PathBuf};
use std::ptr::replace;
use std::{error::Error, io::BufReader, io::Write};
use tokio::fs::create_dir_all;
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
                        .get_feed_file_name(self.feed_urls.get(index).unwrap())
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
                            .get_feed_file_name(self.feed_urls.get(index).unwrap())
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

    fn get_feed_file_name(&self, url: &str) -> String {
        let regex = Regex::new(r"\/\/(.*\..*)\/").unwrap();
        regex
            .captures(url)
            .expect("[!] file wrong idk.")
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace(".", "_")
            .replace("/", "")
            + ".xml"
    }
    fn get_item_file_name(&self, url: &str) -> String {
        let regex = Regex::new(r"//.*\..*/(.*)").unwrap();
        let reg = regex
            .captures(url)
            .expect("no url base")
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace("/", "");

        reg
    }

    async fn download_feed(&self, url: &str) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::get(url) //
            .await?
            .bytes()
            .await?;
        let channel = Channel::read_from(&content[..])?;
        let channel = self.download_content(channel, Some(2)).await.unwrap();

        let mut file = File::create(self.get_feed_file_name(url)).expect("Unable to create file");
        channel.write_to(file);

        Ok(channel)
    }
    async fn download_item_images(
        &self,
        item: &mut Item,
        dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let regex = Regex::new(r#"<img[^>]+src="([^">]+)""#)?;

        // Ensure item.content is a String
        let content = match item.content.as_mut() {
            Some(res) => res,
            None => return Err("[!] no title for feed.".into()),
        };

        // Find all captures in the item content
        let images = regex.captures_iter(content).collect::<Vec<_>>();
        let mut content_clone = content.clone();

        for capture in images {
            // Get the URL from the capture group
            if let Some(url) = capture.get(1) {
                let url = url.as_str();

                // Create a file to save the content
                let file_name = self.get_item_file_name(url);

                let file_path = dir.join(&file_name);
                if file_path.exists() {
                    println!("[!] File with same name in feed already exists")
                } else {
                    // Download the content
                    let data = reqwest::get(url).await?.bytes().await?;
                    let mut file = File::create(&file_path)?;
                    file.write_all(&data)?;
                    content_clone = content_clone.replace(
                        url,
                        file_path
                            .to_str()
                            .expect("[!] created file does not have name"),
                    );
                }
            }
        }
        *content = content_clone;
        Ok(())
    }
    async fn download_item_encsoure(
        &self,
        item: &mut Item,
        dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let url = match item.clone().enclosure {
            Some(closure) => closure.url,
            None => return Err("[!] no cosure".into()),
        };

        // Create a file to save the content
        let file_name = self.get_item_file_name(&url);

        let file_path = dir.join(&file_name);
        if file_path.exists() {
            println!("[!] File with same name in feed already exists")
        } else {
            // Download the content
            let data = reqwest::get(url).await?.bytes().await?;
            let mut file = File::create(&file_path)?;
            file.write_all(&data)?;
            item.enclosure.as_mut().unwrap().url = file_path.to_str().unwrap().to_string();
        }

        Ok(())
    }

    async fn download_content(
        &self,
        mut channel: Channel,
        item_cap: Option<u16>,
    ) -> Result<Channel, Box<dyn Error>> {
        // Create a directory for the channel
        let channel_dir = PathBuf::from(format!(r"podcasts/{}", &channel.title.replace(" ", "_")));
        let _ = create_dir_all(&channel_dir).await.is_err();

        let channel_copy = channel.clone();

        match item_cap {
            Some(cap) => {
                for i in 0..cap {
                    let item = &mut channel.items[i as usize];
                    self.download_item_images(item, channel_dir.clone()).await;
                    self.download_item_encsoure(item, channel_dir.clone()).await;
                }
            }
            None => {
                for item in &mut channel.items {
                    self.download_item_images(item, channel_dir.clone()).await;
                    self.download_item_encsoure(item, channel_dir.clone()).await;
                }
            }
        }

        assert_ne!(channel, channel_copy);

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

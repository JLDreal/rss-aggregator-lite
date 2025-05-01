use std::error::Error;
use std::f32::NAN;
use std::{
    fs::{self, File, create_dir_all},
    io::Write,
    path::PathBuf,
};
pub mod models;
pub mod schema;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::{Connection, SqliteConnection};
use models::*;
use regex::Regex;
use rss::Channel;
use schema::*;
use serde_derive::Deserialize;

use toml;

pub struct RssController {
    pub items: Vec<models::Item>,
    pub feed_urls: Vec<String>,
    pub online: bool,
    conn: SqliteConnection,
}

impl RssController {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            feed_urls: Vec::new(),
            online: false,
            conn: models::establish_connection(),
        }
    }

    pub fn add_feed(&mut self, url: String) {
        if !self.feed_urls.contains(&url) {
            self.feed_urls.push(url);
        }
    }

    pub async fn get_feeds(&mut self) {
        self.items = Vec::new();
        for i in 0..self.feed_urls.len() {
            self.get_feed(i).await;
        }
    }

    pub async fn get_feed(&mut self, index: usize) {
        let mut tmp_channels: Vec<Channel> = Vec::new();
        if self.online {
            match self.load_feed(&self.feed_urls.get(index).unwrap()).await {
                Ok(res) => {
                    todo!("[!] load local items")
                }
                Err(_) => println!("[!] No local version."),
            };
        } else {
            match self.download_feed(self.feed_urls.get(index).unwrap()).await {
                Ok(res) => {
                    todo!("[!] download items");
                }
                Err(_) => match self.load_feed(&self.feed_urls.get(index).unwrap()).await {
                    Ok(res) => {
                        todo!("[!] load local items")
                    }
                    Err(_) => println!("[!] No internet and no local version."),
                },
            };
        }
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

        Ok(channel)
    }
    async fn download_item_images(
        &self,
        item: &mut models::Item,
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
        item: &mut models::Item,
        dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        //
        todo!("[!] get inclosure");
        let url = "tet";
        // let url = match item.clone().enclosure {
        //    Some(closure) => closure.url,
        //    None => return Err("[!] no cosure".into()),
        //};

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
            // item.enclosure.as_mut().unwrap().url = file_path.to_str().unwrap().to_string();
            todo!("[!] enclosure change")
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
        let feed_dir = PathBuf::from("feeds/");

        let _ = create_dir_all(&feed_dir).is_err();
        let _ = create_dir_all(&channel_dir).is_err();

        let channel_copy = channel.clone();
        // convert
        todo!("[!] items from db");
        match item_cap {
            Some(cap) => {
                for i in 0..cap {
                    let item = &mut channel.items[i as usize];
                    // self.download_item_images(item, channel_dir.clone()).await;
                    // self.download_item_encsoure(item, channel_dir.clone()).await;
                }
            }
            None => {
                for item in &mut channel.items {
                    // self.download_item_images(item, channel_dir.clone()).await;
                    // self.download_item_encsoure(item, channel_dir.clone()).await;
                }
            }
        }
        // save item to db
        todo!("[!] save item");

        Ok(channel) // Return the channel after processing
    }

    async fn create_feed(&self, feed_name: &str) -> Result<Channel, Box<dyn Error>> {
        let mut channel = Channel::default();
        //
        todo!("[!] download feed insert items");
        Ok(channel)
    }
    async fn create_item(&mut self, item: rss::Item) -> Result<Item, Box<dyn Error>> {
        //
        let enclosure = NewEnclosure {
            len: &item.enclosure.as_ref().unwrap().length(),
            mime_type: item.enclosure.as_ref().unwrap().mime_type(),
            url: item.enclosure.as_ref().unwrap().url(),
        };
        let enclosure = diesel::insert_into(enclosures::table)
            .values(&enclosure)
            .returning(Enclosure::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new enclosure");

        let mut categories: Vec<Category> = Vec::new();
        item.categories().iter().for_each(|x| {
            let category = NewCategory {
                name: &x.name,
                domain: &x.domain().unwrap(),
            };
            let category = diesel::insert_into(categories::table)
                .values(&category)
                .returning(Category::as_returning())
                .get_result(&mut self.conn)
                .expect("Error saving new enclosure");
            categories.push(category);
        });

        let item = NewItem {
            author: &item.author().unwrap(),
            title: &item.title().unwrap(),
            pub_date: &item.pub_date().unwrap(),
            content: &item.content().unwrap(),
            enclosure_id: &enclosure.id,
        };

        let item = diesel::insert_into(items::table)
            .values(&item)
            .returning(Item::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new enclosure");
        Ok(item)
    }

    async fn load_feed(&self, feed_name: &str) -> Result<Channel, Box<dyn Error>> {
        let mut channel = Channel::default();
        //
        todo!("[!] load feed");
        Ok(channel)
    }
    async fn load_item(&self, item_id: usize) -> Result<Item, Box<dyn Error>> {
        let item: models::Item = Item {
            id: 1,
            title: "test".to_owned(),
            author: None,
            pub_date: None,
            content: None,
            enclosure_id: None,
        };
        todo!("[!] load item");
        Ok(item)
    }

    async fn delete_item(&self, item_id: usize) -> Result<(), Box<dyn Error>> {
        todo!("[!] delete item");
        Ok(())
    }

    async fn delete_feed(&self, feed_name: &str) -> Result<(), Box<dyn Error>> {
        todo!("[!] delete feed");
        Ok(())
    }
}

#[derive(Deserialize)]
struct Settings {
    is_offline: bool,
    cache_amm: usize,
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
                cache_amm: 5,
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

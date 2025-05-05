use std::error::Error;
use std::f32::NAN;
use std::{
    fs::{self, File, create_dir_all},
    io::Write,
    path::PathBuf,
};
pub mod db;
pub mod models;
pub mod schema;
use db::establish_connection;
use diesel::SelectableHelper;
use diesel::{Connection, SqliteConnection};
use diesel::{Insertable, RunQueryDsl};
use enclosure::*;
use enclosures::table;
use models::*;
use regex::Regex;
use rss::Channel;
use schema::*;
use serde_derive::Deserialize;

use toml;

pub trait RssOperations {
    fn add_feed(&mut self, url: String);
    async fn get_feeds(&mut self);
    async fn get_feed(&mut self, index: usize);
    async fn download_feed(&self, url: &str) -> Result<Channel, Box<dyn Error>>;
    async fn load_feed(&self, feed_name: &str) -> Result<Channel, Box<dyn Error>>;
    async fn create_feed(&self, feed_name: &str) -> Result<Channel, Box<dyn Error>>;
    async fn delete_feed(&self, feed_name: &str) -> Result<(), Box<dyn Error>>;
    async fn create_item(&mut self, item: rss::Item) -> Result<Item, Box<dyn Error>>;
    async fn load_item(&self, item_id: usize) -> Result<Item, Box<dyn Error>>;
    async fn delete_item(&self, item_id: usize) -> Result<(), Box<dyn Error>>;
}

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
            conn: db::establish_connection(),
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

    async fn download_item_images(
        &self,
        item: &mut models::Item,
        dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        todo!("Implement image downloading logic for items");
    }

    async fn download_item_enclosure(
        &self,
        item: &mut models::Item,
        dir: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        todo!("Implement enclosure downloading logic");
    }

    async fn download_content(
        &self,
        mut channel: Channel,
        item_cap: Option<u16>,
    ) -> Result<Channel, Box<dyn Error>> {
        todo!("Implement content downloading and processing");
    }
}

impl RssOperations for RssController {
    fn add_feed(&mut self, url: String) {
        if !self.feed_urls.contains(&url) {
            self.feed_urls.push(url);
        }
    }

    async fn get_feeds(&mut self) {
        todo!("Implement fetching all feeds");
    }

    async fn get_feed(&mut self, index: usize) {
        todo!("Implement fetching single feed by index");
    }

    async fn download_feed(&self, url: &str) -> Result<Channel, Box<dyn Error>> {
        todo!("Implement feed downloading from URL");
    }

    async fn load_feed(&self, feed_name: &str) -> Result<Channel, Box<dyn Error>> {
        todo!("Implement loading feed from storage");
    }

    async fn create_feed(&self, feed_name: &str) -> Result<Channel, Box<dyn Error>> {
        todo!("Implement creating new feed");
    }

    async fn create_item(&mut self, item: rss::Item) -> Result<Item, Box<dyn Error>> {
        todo!("Implement creating new item");
    }

    async fn load_item(&self, item_id: usize) -> Result<Item, Box<dyn Error>> {
        todo!("Implement loading item by ID");
    }

    async fn delete_item(&self, item_id: usize) -> Result<(), Box<dyn Error>> {
        todo!("Implement deleting item by ID");
    }

    async fn delete_feed(&self, feed_name: &str) -> Result<(), Box<dyn Error>> {
        todo!("Implement deleting feed by name");
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

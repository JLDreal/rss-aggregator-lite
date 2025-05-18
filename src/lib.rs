mod entities;
use anyhow::Ok;
use entities::category::ActiveModel as DbCategory;
use entities::enclosure::ActiveModel as DbEnclosure;
use entities::item::ActiveModel as DbItem;
use entities::*;
use futures::executor::block_on;
use regex::Regex;
use rss::Channel;
use rss::Item;

use sea_orm::*;
use serde_derive::Deserialize;
use std::error::Error;
use std::{
    fs::{self, File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use toml;
const DATABASE_FILE: &str = "sqlite://database.db?mode=rwc";

pub trait RssOperations {
    fn add_feed(&mut self, url: String);
    fn get_feeds(&mut self) -> impl std::future::Future<Output = ()> + Send;
    fn get_feed(&mut self, index: usize) -> impl std::future::Future<Output = ()> + Send;
    fn download_feed(
        &self,
        url: &str,
    ) -> impl std::future::Future<Output = Result<Channel, anyhow::Error>> + Send;
    fn load_feed(
        &self,
        feed_name: &str,
    ) -> impl std::future::Future<Output = Result<Channel, anyhow::Error>> + Send;
    fn create_feed(
        &self,
        feed_name: &str,
    ) -> impl std::future::Future<Output = Result<Channel, anyhow::Error>> + Send;
    fn delete_feed(
        &self,
        feed_name: &str,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send;
    fn create_item(
        &mut self,
        item: rss::Item,
    ) -> impl std::future::Future<Output = Result<item::Model, anyhow::Error>> + Send;
    fn load_item(
        &self,
        item_id: usize,
    ) -> impl std::future::Future<Output = Result<item::Model, anyhow::Error>> + Send;
    fn delete_item(
        &self,
        item_id: usize,
    ) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send;
}

pub struct RssController {
    pub items: Vec<Item>,
    pub feed_urls: Vec<String>,
    pub online: bool,
}

impl RssController {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            feed_urls: Vec::new(),
            online: false,
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
        item: &mut Item,
        dir: PathBuf,
    ) -> Result<(), anyhow::Error> {
        todo!("Implement image downloading logic for items");
    }

    async fn download_item_enclosure(
        &self,
        item: &mut Item,
        dir: PathBuf,
    ) -> Result<(), anyhow::Error> {
        todo!("Implement enclosure downloading logic");
    }

    async fn download_content(
        &self,
        mut channel: Channel,
        item_cap: Option<u16>,
    ) -> Result<Channel, anyhow::Error> {
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

    async fn download_feed(&self, url: &str) -> Result<Channel, anyhow::Error> {
        todo!("Implement feed downloading from URL");
    }

    async fn load_feed(&self, feed_name: &str) -> Result<Channel, anyhow::Error> {
        todo!("Implement loading feed from storage");
    }

    async fn create_feed(&self, feed_name: &str) -> Result<Channel, anyhow::Error> {
        todo!("Implement creating new feed");
    }

    async fn create_item(&mut self, item: rss::Item) -> Result<item::Model, anyhow::Error> {
        let db = Database::connect(DATABASE_FILE).await?;
        let item = DbItem {
            id: sea_orm::ActiveValue::NotSet,
            title: ActiveValue::set(item.title().unwrap().to_owned()),
            author: ActiveValue::set(item.author().unwrap().to_owned()),
            content: ActiveValue::set(item.content().unwrap().to_owned()),
            link: ActiveValue::set(item.link().unwrap().to_owned()),
            description: ActiveValue::set(item.description().unwrap().to_owned()),
            pub_date: ActiveValue::set(item.pub_date().unwrap().to_owned()),
            enclosure: ActiveValue::NotSet,
            categories: ActiveValue::NotSet,
        };
        Ok(DbItem::insert(item, &db).await.unwrap())
    }

    async fn load_item(&self, item_id: usize) -> Result<item::Model, anyhow::Error> {
        todo!("Implement loading item by ID");
    }

    async fn delete_item(&self, item_id: usize) -> Result<(), anyhow::Error> {
        todo!("Implement deleting item by ID");
    }

    async fn delete_feed(&self, feed_name: &str) -> Result<(), anyhow::Error> {
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
    pub async fn load(&mut self) -> Result<(), anyhow::Error> {
        let content = fs::read_to_string("settings/settings.toml")?;
        let data: Data = toml::from_str(&content)?;

        self.settings = data.settings;
        self.rss_controller.feed_urls = data.feed.feeds;
        self.rss_controller.online = self.settings.is_offline.clone();
        self.rss_controller.get_feeds().await;

        Ok(())
    }
}

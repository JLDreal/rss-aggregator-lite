use rss::Channel;
use std::iter;
use std::{error::Error, io::Write, io::BufReader};
use std::fs::File;
use regex::Regex;

pub struct RssController {
    pub channels: Vec<Channel>,
    pub feed_urls: Vec<String>,
    pub online: bool

}

impl RssController {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            feed_urls: Vec::new(),
            online: false
        }
    }

    pub async fn add_feed(&mut self, url: String) {
        if !self.feed_urls.contains(&url) {
            let new_feed = match self.download_feed(&url).await {
                Ok(file) => file,
                Err(error) => panic!("Problem downloading file: {error:?}"),
            };
            self.channels.push(new_feed);
            self.feed_urls.push(url);
        }
    }

    pub async fn get_feeds(&self){
        if self.online {
            self.download_feed(self.feed_urls.get(0).unwrap()).await;
        }
        else{
            self.load_feed(format!("{}.xml",&self.get_file_name(self.feed_urls.get(0).unwrap())).as_str()).await;
        }
    }
    fn get_file_name(&self, url: &str) -> String {
       let regex = Regex::new(r"\/\/(.*)\/").unwrap();
       regex.captures(url).expect("no url base").get(0).unwrap().as_str().to_string().replace(".", "_").replace("/", "")

    } 
    async fn download_feed(&self, url: &str) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::get(url)//  
            .await?
            .bytes()
            .await?;
        let channel = Channel::read_from(&content[..])?;
        let mut file = File::create(format!("{}.xml",self.get_file_name(url))).expect("Unable to create file"); 
        file.write(&content)?;
        
    
        Ok(channel)
    }

    async fn load_feed(&self, filename: &str ) -> Result<Channel, Box<dyn Error>> {
        let file = File::open(filename).unwrap();
        let channel = Channel::read_from(BufReader::new(file)).unwrap();
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



use core::RssController;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut c = RssController::new();
    c.add_feed("https://lobste.rs/rss".to_string()).await;
    c.get_feeds().await;
    c.get_feeds().await;
}

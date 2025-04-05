use core::RssController;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut c = RssController::new();
    c.add_feed("https://lobste.rs/rss".to_string()).await;
    c.add_feed("https://www.tagesschau.de/infoservices/alle-meldungen-100~rss2.xml".to_string())
        .await;
    c.get_feeds().await;
    c.get_feeds().await;
}

use core::SettingsController;

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

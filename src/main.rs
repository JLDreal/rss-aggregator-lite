use core::SettingsController;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut settings = SettingsController::new();
    settings.load().await.unwrap();
    settings
        .rss_controller
        .items
        .iter()
        .for_each(|x| println!("title: {:?}", x.title));
    let feed = settings
        .rss_controller
        .items
        .iter()
        .find(|x| x.title. == "2.5 Admins")
        .unwrap();
}

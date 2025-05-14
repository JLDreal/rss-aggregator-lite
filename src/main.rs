mod entities;

use entities::{prelude::*, *};

use futures::executor::block_on;
use sea_orm::*;

const DATABASE_FILE: &str = "sqlite://database.db?mode=rwc";

async fn run() -> Result<(), anyhow::Error> {
    let db = Database::connect(DATABASE_FILE).await?;
    let category = category::ActiveModel {
        id: ActiveValue::set(1),
        name: ActiveValue::set("test".to_owned()),
        domain: ActiveValue::set("test.com".to_owned()),
    };
    let res = Category::insert(category).exec(&db).await?;
    print!("{:?}", res);
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}

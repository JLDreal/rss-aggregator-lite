pub use sea_orm_migration::prelude::*;

mod m20250514_154948_create_category_table;
mod m20250514_161012_create_enclosure_table;
mod m20250514_161015_create_item_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250514_154948_create_category_table::Migration),
            Box::new(m20250514_161012_create_enclosure_table::Migration),
            Box::new(m20250514_161015_create_item_table::Migration),
        ]
    }
}
pub struct ItemService;

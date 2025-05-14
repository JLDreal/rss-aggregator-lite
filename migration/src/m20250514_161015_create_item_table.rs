use std::any::Any;

use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250514_154948_create_category_table::Category,
    m20250514_161012_create_enclosure_table::Enclosure,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(pk_auto(Item::Id))
                    .col(string(Item::Title))
                    .col(string(Item::Link))
                    .col(string(Item::Author))
                    .col(string(Item::Description))
                    .col(string(Item::PubDate))
                    .col(string(Item::Content))
                    .col(integer(Item::Enclosure))
                    .col(integer(Item::Categories))
                    .foreign_key(
                        ForeignKey::create()
                            .name("enclosure")
                            .from(Item::Table, Item::Enclosure)
                            .to(Enclosure::Table, Enclosure::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("categories")
                            .from(Item::Table, Item::Categories)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Item {
    Table,
    Id,
    Title,
    Link,
    Author,
    Description,
    Categories,
    Enclosure,
    PubDate,
    Content,
}

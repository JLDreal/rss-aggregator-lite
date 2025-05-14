use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Enclosure::Table)
                    .if_not_exists()
                    .col(pk_auto(Enclosure::Id))
                    .col(string(Enclosure::Length))
                    .col(string(Enclosure::MimeType))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Enclosure::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Enclosure {
    Table,
    Id,
    Length,
    MimeType,
}

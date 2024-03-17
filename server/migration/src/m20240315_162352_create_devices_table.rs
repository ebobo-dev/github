use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Devices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Devices::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Devices::Fingerprint).string().not_null().unique_key())
                    .col(ColumnDef::new(Devices::Fighter).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Devices::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Devices {
    Table,
    Id,
    Fingerprint,
    Fighter,
}

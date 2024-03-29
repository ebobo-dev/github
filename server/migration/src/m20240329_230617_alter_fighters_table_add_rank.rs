use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Fighters::Table)
                    .add_column(ColumnDef::new(Fighters::Rank).integer().not_null())
                    .add_column(ColumnDef::new(Fighters::Device).string().not_null())
                    .drop_column(Fighters::Fingerprint)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Fighters::Table)
                    .drop_column(Fighters::Rank)
                    .drop_column(Fighters::Device)
                    .add_column(ColumnDef::new(Fighters::Fingerprint).string().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Fighters {
    Table,
    Fingerprint,
    Device,
    Rank,
}

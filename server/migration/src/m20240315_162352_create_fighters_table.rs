use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Fighters::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fighters::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Fighters::Fingerprint).string().not_null().unique_key())
                    .col(ColumnDef::new(Fighters::Fighter).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Fighters::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Fighters {
    Table,
    Id,
    Fingerprint,
    Fighter,
}

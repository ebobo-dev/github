use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Requests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Requests::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Requests::Address).string().null())
                    .col(ColumnDef::new(Requests::Created).date_time().not_null())
                    .col(ColumnDef::new(Requests::Fingerprint).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Requests::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Requests {
    Table,
    Id,
    Address,
    Created,
    Fingerprint,
}

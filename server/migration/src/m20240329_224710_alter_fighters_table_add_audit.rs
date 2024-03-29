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
                    .add_column(ColumnDef::new(Fighters::Created).date_time().not_null())
                    .add_column(ColumnDef::new(Fighters::Root).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Fighters::Table)
                    .drop_column(Fighters::Created)
                    .drop_column(Fighters::Root)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Fighters {
    Table,
    Created,
    Root,
}

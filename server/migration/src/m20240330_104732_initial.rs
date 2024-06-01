use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Users::Fingerprint)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Fighter)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Rank).integer().not_null().default(0))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Matches::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Matches::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Matches::Left).string().not_null())
                    .col(ColumnDef::new(Matches::Right).string().not_null())
                    .col(ColumnDef::new(Matches::Result).string().null())
                    .col(ColumnDef::new(Matches::Date).date_time().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Matches::Table)
                            .to_tbl(Users::Table)
                            .from_col(Matches::Left)
                            .to_col(Users::Fingerprint),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Matches::Table)
                            .to_tbl(Users::Table)
                            .from_col(Matches::Right)
                            .to_col(Users::Fingerprint),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Matches::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Fingerprint,
    Fighter,
    Rank,
}

#[derive(DeriveIden)]
enum Matches {
    Table,
    Id,
    Left,
    Right,
    Result,
    Date,
}

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
                        ColumnDef::new(Fighters::Fingerprint)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Fighters::Emo)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Fighters::Rank).integer().not_null().default(0))
                    .col(ColumnDef::new(Fighters::Queue).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Matches::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Matches::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Matches::Winner).string().null())
                    .col(ColumnDef::new(Matches::Date).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Matches::Table)
                            .to_tbl(Fighters::Table)
                            .from_col(Matches::Winner)
                            .to_col(Fighters::Fingerprint),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Plays::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Plays::Fighter).string().not_null().primary_key())
                    .col(ColumnDef::new(Plays::Match).uuid().not_null().primary_key())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Plays::Table)
                            .to_tbl(Fighters::Table)
                            .from_col(Plays::Fighter)
                            .to_col(Fighters::Fingerprint),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Fighters::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Matches::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Plays::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Fighters {
    Table,
    Fingerprint,
    Emo,
    Rank,
    Queue,
}

#[derive(DeriveIden)]
enum Matches {
    Table,
    Id,
    Winner,
    Date,
}

#[derive(DeriveIden)]
enum Plays {
    Table,
    Match,
    Fighter,
}

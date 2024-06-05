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
                    .col(ColumnDef::new(Matches::Result).string().not_null())
                    .col(ColumnDef::new(Matches::Date).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Matches::Table)
                            .to_tbl(Fighters::Table)
                            .from_col(Matches::Left)
                            .to_col(Fighters::Fingerprint),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Matches::Table)
                            .to_tbl(Fighters::Table)
                            .from_col(Matches::Right)
                            .to_col(Fighters::Fingerprint),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Queue::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Queue::Fingerprint).string().not_null().primary_key())
                    .col(ColumnDef::new(Queue::Date).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Queue::Table)
                            .to_tbl(Fighters::Table)
                            .from_col(Queue::Fingerprint)
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
            .drop_table(Table::drop().table(Queue::Table).to_owned())
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

#[derive(DeriveIden)]
enum Queue {
    Table,
    Fingerprint,
    Date,
}

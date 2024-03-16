use super::m20240315_162352_create_devices_table::*;
use super::m20240316_124623_create_locations_table::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DevicesLocations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DevicesLocations::LocationId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DevicesLocations::DeviceId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("idx_device_location_device")
                            .col(DevicesLocations::DeviceId)
                            .col(DevicesLocations::LocationId)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_device_location_device")
                            .from(DevicesLocations::Table, DevicesLocations::DeviceId)
                            .to(Devices::Table, Devices::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_device_location_location")
                            .from(DevicesLocations::Table, DevicesLocations::LocationId)
                            .to(Locations::Table, Locations::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DevicesLocations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DevicesLocations {
    Table,
    LocationId,
    DeviceId,
}

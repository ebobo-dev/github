pub use sea_orm_migration::prelude::*;

mod m20240315_162352_create_devices_table;
mod m20240316_124623_create_locations_table;
mod m20240316_124859_create_devices_locations_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240315_162352_create_devices_table::Migration),
            Box::new(m20240316_124623_create_locations_table::Migration),
            Box::new(m20240316_124859_create_devices_locations_table::Migration),
        ]
    }
}

pub use sea_orm_migration::prelude::*;

mod m20240315_162352_create_fighters_table;
mod m20240329_205732_create_requests_table;
mod m20240329_224710_alter_fighters_table_add_audit;
mod m20240329_230617_alter_fighters_table_add_rank;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240315_162352_create_fighters_table::Migration),
            Box::new(m20240329_205732_create_requests_table::Migration),
            Box::new(m20240329_224710_alter_fighters_table_add_audit::Migration),
            Box::new(m20240329_230617_alter_fighters_table_add_rank::Migration),
        ]
    }
}

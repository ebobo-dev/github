//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "locations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::devices_locations::Entity")]
    DevicesLocations,
}

impl Related<super::devices_locations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DevicesLocations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

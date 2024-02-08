//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "processing_raw_material_inventory")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub raw_material_inventory_id: i32,
    #[sea_orm(unique)]
    pub uuid: String,
    pub name: String,
    pub specification: String,
    pub quantity: i32,
    pub unit: String,
    pub purchase_at: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "seeds")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub seed_id: i32,
    pub seed_name: String,
    pub supplier: String,
    pub shelf_life: i32,
    pub quantity: i32,
    #[sea_orm(column_type = "Text")]
    pub remark: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::planting_records::Entity")]
    PlantingRecords,
}

impl Related<super::planting_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlantingRecords.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

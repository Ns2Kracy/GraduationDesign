//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "analysis")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub analysis_id: i32,
    pub analysis_date: Date,
    pub analysis_type: String,
    #[sea_orm(column_type = "Text")]
    pub analysis_content: String,
    #[sea_orm(column_type = "Text")]
    pub analysis_result: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
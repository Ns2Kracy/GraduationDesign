//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "alert_information")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub alert_id: i32,
    pub alert_type: String,
    pub alert_date: Date,
    #[sea_orm(column_type = "Text")]
    pub alert_content: String,
    pub alert_level: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

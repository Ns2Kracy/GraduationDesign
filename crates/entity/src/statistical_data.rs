//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "statistical_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub statistical_id: i32,
    pub statistical_date: Date,
    pub statistical_data_type: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub statistical_data_value: Decimal,
    #[sea_orm(column_type = "Text")]
    pub remark: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

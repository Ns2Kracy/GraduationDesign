//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pesticide")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: String,
    pub crops: String,
    pub name: String,
    pub r#type: String,
    pub number: i32,
    pub unit: String,
    pub price: Decimal,
    pub description: String,
    pub image: String,
    pub source: String,
    pub supplier: String,
    pub apply_at: Date,
    pub application_method: String,
    pub quantity: Decimal,
    pub employee_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::employee::Entity",
        from = "Column::EmployeeId",
        to = "super::employee::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Employee,
}

impl Related<super::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

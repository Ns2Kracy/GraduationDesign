//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "planting_harvest")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub harvest_id: i32,
    #[sea_orm(unique)]
    pub uuid: String,
    pub crop: String,
    pub harvest_at: Date,
    pub quantity: i32,
    pub employee_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::planting_employee::Entity",
        from = "Column::EmployeeId",
        to = "super::planting_employee::Column::EmployeeId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PlantingEmployee,
}

impl Related<super::planting_employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlantingEmployee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

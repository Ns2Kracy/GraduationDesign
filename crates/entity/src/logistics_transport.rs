//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "logistics_transport")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub record_id: i32,
    #[sea_orm(unique)]
    pub uuid: String,
    pub product_name: String,
    pub transport_at: Date,
    pub transport_method: String,
    pub transport_status: String,
    pub destination: String,
    pub express_id: i32,
    pub warehouse_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::logistics_express::Entity",
        from = "Column::ExpressId",
        to = "super::logistics_express::Column::ExpressId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    LogisticsExpress,
    #[sea_orm(
        belongs_to = "super::logistics_warehouse::Entity",
        from = "Column::WarehouseId",
        to = "super::logistics_warehouse::Column::WarehouseId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    LogisticsWarehouse,
}

impl Related<super::logistics_express::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogisticsExpress.def()
    }
}

impl Related<super::logistics_warehouse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogisticsWarehouse.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub last_login_at: Option<DateTime>,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::operation_logs::Entity")]
    OperationLogs,
    #[sea_orm(has_many = "super::query_logs::Entity")]
    QueryLogs,
}

impl Related<super::operation_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OperationLogs.def()
    }
}

impl Related<super::query_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QueryLogs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

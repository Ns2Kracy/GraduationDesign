use sea_orm_migration::prelude::*;

use crate::m20240321_165701_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(QueryLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QueryLogs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QueryLogs::UserId).integer().not_null())
                    .col(ColumnDef::new(QueryLogs::QueryDate).date().not_null())
                    .col(ColumnDef::new(QueryLogs::QueryContent).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_query_logs_user_id")
                            .from(QueryLogs::Table, QueryLogs::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QueryLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum QueryLogs {
    #[sea_orm(iden = "query_logs")]
    Table,
    #[sea_orm(iden = "query_log_id")]
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "query_date")]
    QueryDate,
    #[sea_orm(iden = "query_content")]
    QueryContent,
}

use sea_orm_migration::prelude::*;

use crate::{
    m20240321_163618_create_farms_table::Farms, m20240321_165701_create_users_table::Users,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OperationLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OperationLogs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OperationLogs::UserId).integer().not_null())
                    .col(ColumnDef::new(OperationLogs::LogDate).date().not_null())
                    .col(ColumnDef::new(OperationLogs::LogContent).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_operation_logs_user_id")
                            .from(OperationLogs::Table, OperationLogs::UserId)
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
            .drop_table(Table::drop().table(OperationLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OperationLogs {
    #[sea_orm(iden = "operation_logs")]
    Table,
    #[sea_orm(iden = "log_id")]
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "log_date")]
    LogDate,
    #[sea_orm(iden = "log_content")]
    LogContent,
}

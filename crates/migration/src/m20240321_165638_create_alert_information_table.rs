use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AlertInformation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AlertInformation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AlertInformation::AlertType)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AlertInformation::AlertDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AlertInformation::AlertContent)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AlertInformation::AlertLevel)
                            .string_len(20)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AlertInformation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AlertInformation {
    #[sea_orm(iden = "alert_information")]
    Table,
    #[sea_orm(iden = "alert_id")]
    Id,
    #[sea_orm(iden = "alert_type")]
    AlertType,
    #[sea_orm(iden = "alert_date")]
    AlertDate,
    #[sea_orm(iden = "alert_content")]
    AlertContent,
    #[sea_orm(iden = "alert_level")]
    AlertLevel,
}

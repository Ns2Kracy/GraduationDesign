use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Report::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Report::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Report::ReportDate).date().not_null())
                    .col(ColumnDef::new(Report::ReportContent).text().not_null())
                    .col(ColumnDef::new(Report::ReportType).string_len(50).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Report::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Report {
    #[sea_orm(iden = "report")]
    Table,
    #[sea_orm(iden = "report_id")]
    Id,
    #[sea_orm(iden = "report_date")]
    ReportDate,
    #[sea_orm(iden = "report_content")]
    ReportContent,
    #[sea_orm(iden = "report_type")]
    ReportType,
}

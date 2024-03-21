use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Analysis::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Analysis::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Analysis::AnalysisDate).date().not_null())
                    .col(
                        ColumnDef::new(Analysis::AnalysisType)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Analysis::AnalysisContent).text().not_null())
                    .col(ColumnDef::new(Analysis::AnalysisResult).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Analysis::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Analysis {
    #[sea_orm(iden = "analysis")]
    Table,
    #[sea_orm(iden = "analysis_id")]
    Id,
    #[sea_orm(iden = "analysis_date")]
    AnalysisDate,
    #[sea_orm(iden = "analysis_type")]
    AnalysisType,
    #[sea_orm(iden = "analysis_content")]
    AnalysisContent,
    #[sea_orm(iden = "analysis_result")]
    AnalysisResult,
}

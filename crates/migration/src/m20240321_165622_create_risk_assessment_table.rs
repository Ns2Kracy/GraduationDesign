use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RiskAssessment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RiskAssessment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RiskAssessment::AssessmentType)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RiskAssessment::AssessmentDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RiskAssessment::RiskLevel)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(ColumnDef::new(RiskAssessment::AlertInfo).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RiskAssessment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RiskAssessment {
    #[sea_orm(iden = "risk_assessment")]
    Table,
    #[sea_orm(iden = "assessment_id")]
    Id,
    #[sea_orm(iden = "assessment_type")]
    AssessmentType,
    #[sea_orm(iden = "assessment_date")]
    AssessmentDate,
    #[sea_orm(iden = "risk_level")]
    RiskLevel,
    #[sea_orm(iden = "alert_info")]
    AlertInfo,
}

use sea_orm_migration::prelude::*;

use crate::m20240321_163618_create_farms_table::Farms;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EnforcementRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EnforcementRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EnforcementRecords::FarmId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EnforcementRecords::InspectionDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EnforcementRecords::InspectionName)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EnforcementRecords::InspectionContent)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EnforcementRecords::InspectionResult)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EnforcementRecords::PenaltyInfo)
                            .text()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_enforcement_records_farm_id")
                            .from(EnforcementRecords::Table, EnforcementRecords::FarmId)
                            .to(Farms::Table, Farms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EnforcementRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EnforcementRecords {
    #[sea_orm(iden = "enforcement_records")]
    Table,
    #[sea_orm(iden = "enforcement_record_id")]
    Id,
    #[sea_orm(iden = "farm_id")]
    FarmId,
    #[sea_orm(iden = "inspection_date")]
    InspectionDate,
    #[sea_orm(iden = "inspection_name")]
    InspectionName,
    #[sea_orm(iden = "inspection_content")]
    InspectionContent,
    #[sea_orm(iden = "inspection_result")]
    InspectionResult,
    #[sea_orm(iden = "penalty_info")]
    PenaltyInfo,
}

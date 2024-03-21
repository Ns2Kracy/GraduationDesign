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
                    .table(PesticideResidueDetectionRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PesticideResidueDetectionRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PesticideResidueDetectionRecords::FarmId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideResidueDetectionRecords::CropName)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideResidueDetectionRecords::DetectionDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideResidueDetectionRecords::PesticideName)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideResidueDetectionRecords::ResidueLevel)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideResidueDetectionRecords::DetectionMethod)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideResidueDetectionRecords::Remark)
                            .text()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_pesticide_residue_detection_records_farm_id")
                            .from(
                                PesticideResidueDetectionRecords::Table,
                                PesticideResidueDetectionRecords::FarmId,
                            )
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
            .drop_table(
                Table::drop()
                    .table(PesticideResidueDetectionRecords::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PesticideResidueDetectionRecords {
    #[sea_orm(iden = "pesticide_residue_detection_records")]
    Table,
    #[sea_orm(iden = "pesticide_residue_id")]
    Id,
    #[sea_orm(iden = "farm_id")]
    FarmId,
    #[sea_orm(iden = "crop_name")]
    CropName,
    #[sea_orm(iden = "detection_date")]
    DetectionDate,
    #[sea_orm(iden = "pesticide_name")]
    PesticideName,
    #[sea_orm(iden = "residue_level")]
    ResidueLevel,
    #[sea_orm(iden = "detection_method")]
    DetectionMethod,
    #[sea_orm(iden = "remark")]
    Remark,
}

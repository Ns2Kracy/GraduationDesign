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
                    .table(ProductInspectionRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductInspectionRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProductInspectionRecords::FarmId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductInspectionRecords::InspectionDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductInspectionRecords::InspectionResult)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductInspectionRecords::Remark)
                            .text()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_product_inspection_records_farm_id")
                            .from(
                                ProductInspectionRecords::Table,
                                ProductInspectionRecords::FarmId,
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
                    .table(ProductInspectionRecords::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ProductInspectionRecords {
    #[sea_orm(iden = "product_inspection_records")]
    Table,
    #[sea_orm(iden = "inspection_id")]
    Id,
    #[sea_orm(iden = "farm_id")]
    FarmId,
    #[sea_orm(iden = "crop_name")]
    CropName,
    #[sea_orm(iden = "inspection_date")]
    InspectionDate,
    #[sea_orm(iden = "inspection_result")]
    InspectionResult,
    #[sea_orm(iden = "remark")]
    Remark,
}

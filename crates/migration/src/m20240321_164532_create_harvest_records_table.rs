use crate::m20240321_163618_create_farms_table::Farms as FarmTable;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HarvestRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HarvestRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(HarvestRecords::FarmId).integer().not_null())
                    .col(
                        ColumnDef::new(HarvestRecords::CropName)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HarvestRecords::HarvestDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HarvestRecords::HarvestAmount)
                            .float()
                            .not_null(),
                    )
                    .col(ColumnDef::new(HarvestRecords::Remark).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_harvest_records_farm_id")
                            .from(HarvestRecords::Table, HarvestRecords::FarmId)
                            .to(FarmTable::Table, FarmTable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HarvestRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum HarvestRecords {
    #[sea_orm(iden = "harvest_records")]
    Table,
    #[sea_orm(iden = "harvest_id")]
    Id,
    #[sea_orm(iden = "farm_id")]
    FarmId,
    #[sea_orm(iden = "crop_name")]
    CropName,
    #[sea_orm(iden = "harvest_date")]
    HarvestDate,
    #[sea_orm(iden = "harvest_amount")]
    HarvestAmount,
    #[sea_orm(iden = "remark")]
    Remark,
}

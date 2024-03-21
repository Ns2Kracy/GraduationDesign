use sea_orm_migration::prelude::*;

use crate::{
    m20240321_163618_create_farms_table::Farms, m20240321_163642_create_seeds_table::Seeds,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlantingRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantingRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PlantingRecords::FarmId).integer().not_null())
                    .col(ColumnDef::new(PlantingRecords::SeedId).integer().not_null())
                    .col(
                        ColumnDef::new(PlantingRecords::CropName)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingRecords::PlantingDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingRecords::PlantedArea)
                            .float()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlantingRecords::Remark).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_planting_records_farm_id")
                            .from(PlantingRecords::Table, PlantingRecords::FarmId)
                            .to(Farms::Table, Farms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_planting_records_seed_id")
                            .from(PlantingRecords::Table, PlantingRecords::SeedId)
                            .to(Seeds::Table, Seeds::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlantingRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PlantingRecords {
    #[sea_orm(iden = "planting_records")]
    Table,
    #[sea_orm(iden = "planting_record_id")]
    Id,
    #[sea_orm(iden = "farm_id")]
    FarmId,
    #[sea_orm(iden = "seed_id")]
    SeedId,
    #[sea_orm(iden = "crop_name")]
    CropName,
    #[sea_orm(iden = "planting_date")]
    PlantingDate,
    #[sea_orm(iden = "planted_area")]
    PlantedArea,
    #[sea_orm(iden = "remark")]
    Remark,
}

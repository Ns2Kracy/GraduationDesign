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
                    .table(PesticideUsageRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PesticideUsageRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PesticideUsageRecords::FarmId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideUsageRecords::PesticideName)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideUsageRecords::UsageDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideUsageRecords::UsageAmount)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideUsageRecords::UsagePurpose)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PesticideUsageRecords::Remark)
                            .text()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_pesticide_usage_records_farm_id")
                            .from(PesticideUsageRecords::Table, PesticideUsageRecords::FarmId)
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
            .drop_table(Table::drop().table(PesticideUsageRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PesticideUsageRecords {
    #[sea_orm(table = "pesticide_usage_records")]
    Table,
    #[sea_orm(iden = "pesticide_usage_id")]
    Id,
    #[sea_orm(iden = "farm_id")]
    FarmId,
    #[sea_orm(iden = "pesticide_name")]
    PesticideName,
    #[sea_orm(iden = "pesticide_usage_date")]
    UsageDate,
    #[sea_orm(iden = "pesticide_usage_amount")]
    UsageAmount,
    #[sea_orm(iden = "pesticide_usage_purpose")]
    UsagePurpose,
    #[sea_orm(iden = "remark")]
    Remark,
}

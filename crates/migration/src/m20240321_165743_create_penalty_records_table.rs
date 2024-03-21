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
                    .table(PenaltyRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PenaltyRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PenaltyRecords::PenaltyDate)
                            .date()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PenaltyRecords::FarmId).integer().not_null())
                    .col(
                        ColumnDef::new(PenaltyRecords::PenaltyType)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PenaltyRecords::PenaltyAmount)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PenaltyRecords::PenaltyReason)
                            .text()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_penalty_records_farm_id")
                            .from(PenaltyRecords::Table, PenaltyRecords::FarmId)
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
            .drop_table(Table::drop().table(PenaltyRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PenaltyRecords {
    #[sea_orm(iden = "penalty_records")]
    Table,
    #[sea_orm(iden = "penalty_record_id")]
    Id,
    #[sea_orm(iden = "penalty_date")]
    PenaltyDate,
    #[sea_orm(iden = "farm_id")]
    FarmId,
    #[sea_orm(iden = "penalty_type")]
    PenaltyType,
    #[sea_orm(iden = "penalty_amount")]
    PenaltyAmount,
    #[sea_orm(iden = "penalty_reason")]
    PenaltyReason,
}

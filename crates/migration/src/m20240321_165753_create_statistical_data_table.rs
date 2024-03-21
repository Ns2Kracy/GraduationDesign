use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StatisticalData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StatisticalData::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(StatisticalData::StatisticalDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StatisticalData::StatisticalDataType)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StatisticalData::StatisticalDataValue)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(StatisticalData::Remark).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StatisticalData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StatisticalData {
    #[sea_orm(iden = "statistical_data")]
    Table,
    #[sea_orm(iden = "statistical_id")]
    Id,
    #[sea_orm(iden = "statistical_date")]
    StatisticalDate,
    #[sea_orm(iden = "statistical_data_type")]
    StatisticalDataType,
    #[sea_orm(iden = "statistical_data_value")]
    StatisticalDataValue,
    #[sea_orm(iden = "remark")]
    Remark,
}

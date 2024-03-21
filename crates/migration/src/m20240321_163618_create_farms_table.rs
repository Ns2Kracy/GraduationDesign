use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Farms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Farms::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Farms::FarmName).string_len(100).not_null())
                    .col(ColumnDef::new(Farms::Location).string_len(200).not_null())
                    .col(ColumnDef::new(Farms::Area).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Farms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Farms {
    #[sea_orm(iden = "farms")]
    Table,
    #[sea_orm(iden = "farm_id")]
    Id,
    #[sea_orm(iden = "farm_name")]
    FarmName,
    #[sea_orm(iden = "location")]
    Location,
    #[sea_orm(iden = "area")]
    Area,
}

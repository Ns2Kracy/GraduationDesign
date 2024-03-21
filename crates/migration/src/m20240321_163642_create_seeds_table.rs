use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Seeds::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Seeds::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Seeds::SeedName).string_len(100).not_null())
                    .col(ColumnDef::new(Seeds::Supplier).string_len(100).not_null())
                    .col(ColumnDef::new(Seeds::ShelfLife).integer().not_null())
                    .col(ColumnDef::new(Seeds::Quantity).integer().not_null())
                    .col(ColumnDef::new(Seeds::Remark).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Seeds::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Seeds {
    #[sea_orm(iden = "seeds")]
    Table,
    #[sea_orm(iden = "seed_id")]
    Id,
    #[sea_orm(iden = "seed_name")]
    SeedName,
    #[sea_orm(iden = "supplier")]
    Supplier,
    #[sea_orm(iden = "shelf_life")]
    ShelfLife,
    #[sea_orm(iden = "quantity")]
    Quantity,
    #[sea_orm(iden = "remark")]
    Remark,
}

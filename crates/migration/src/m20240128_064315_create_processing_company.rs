use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProcessingStep::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProcessingStep::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProcessingStep::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(ProcessingStep::Name).string().not_null())
                    .col(
                        ColumnDef::new(ProcessingStep::RequiredRawMaterial)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingStep::ProcessingMethod)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingStep::ProcessingAt)
                            .date()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProcessingProduct::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProcessingProduct::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProcessingProduct::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(ProcessingProduct::Name).string().not_null())
                    .col(
                        ColumnDef::new(ProcessingProduct::Specification)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingProduct::ProductAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingProduct::ProductionBatch)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingProduct::ProductionNumber)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingProduct::BatchNumber)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingProduct::ExpireAt)
                            .date()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RawMaterialInventory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RawMaterialInventory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RawMaterialInventory::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(RawMaterialInventory::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RawMaterialInventory::Specification)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RawMaterialInventory::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RawMaterialInventory::Unit)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RawMaterialInventory::PurchaseAt)
                            .date()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 加工流程管理表
        manager
            .drop_table(Table::drop().table(ProcessingStep::Table).to_owned())
            .await?;
        // 加工产品信息表
        manager
            .drop_table(Table::drop().table(ProcessingProduct::Table).to_owned())
            .await?;
        // 原料库存管理表
        manager
            .drop_table(Table::drop().table(RawMaterialInventory::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// 加工流程管理表
/// 用于存储加工流程的基本信息
/// Table: 表名
/// Id: 自增主键
/// Uuid: 唯一标识
/// Name: 加工流程名称
/// RequiredRawMaterial: 所需原料
/// ProcessingMethod: 加工方法
/// ProcessingAt: 加工时间
#[derive(DeriveIden)]
enum ProcessingStep {
    Table,
    Id,
    Uuid,
    Name,
    RequiredRawMaterial,
    ProcessingMethod,
    ProcessingAt,
}

///加工产品信息表
/// 用于存储加工产品的基本信息
/// Table: 表名
/// Id: 自增主键
/// Uuid: 唯一标识
/// Name: 加工产品名称
/// Specification: 规格
/// ProductAt: 生产时间
/// ProductionBatch: 生产批次
/// ProductionNumber: 生产数量
/// BatchNumber: 批号
/// ExpireAt: 有效期
#[derive(DeriveIden)]
enum ProcessingProduct {
    Table,
    Id,
    Uuid,
    Name,
    Specification,
    ProductAt,
    ProductionBatch,
    ProductionNumber,
    BatchNumber,
    ExpireAt,
}

/// 原料库存管理表
/// 用于存储原料库存的基本信息
/// Table: 表名
/// Id: 自增主键
/// Uuid: 唯一标识
/// Name: 原料名称
/// Specification: 规格
/// Quantity: 数量
/// Unit: 单位
/// PurchaseAt: 采购时间
#[derive(DeriveIden)]
enum RawMaterialInventory {
    Table,
    Id,
    Uuid,
    Name,
    Specification,
    Quantity,
    Unit,
    PurchaseAt,
}

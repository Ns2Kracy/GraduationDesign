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
                    .table(ProcessingRawMaterialInventory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProcessingRawMaterialInventory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProcessingRawMaterialInventory::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(ProcessingRawMaterialInventory::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingRawMaterialInventory::Specification)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingRawMaterialInventory::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingRawMaterialInventory::Unit)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProcessingRawMaterialInventory::PurchaseAt)
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
            .drop_table(
                Table::drop()
                    .table(ProcessingRawMaterialInventory::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// 加工流程管理表
/// 用于存储加工流程的基本信息
/// Table: processing_step
/// Columns:
///    id: serial primary key not null
///    uuid: string unique not null
///    name: string not null
///    required_raw_material: string not null
///    processing_method: string not null
///    processing_at: date not null
#[derive(DeriveIden)]
enum ProcessingStep {
    // 表名
    #[sea_orm(iden = "processing_step")]
    Table,
    // 主键
    #[sea_orm(iden = "step_id")]
    Id,
    // 唯一标识
    Uuid,
    // 加工流程名称
    Name,
    // 所需原料
    RequiredRawMaterial,
    // 加工方法
    ProcessingMethod,
    // 加工时间
    ProcessingAt,
}

///加工产品信息表
/// 用于存储加工产品的基本信息
/// Table: processing_product
/// Columns:
///    id: serial primary key not null
///    uuid: string unique not null
///    name: string not null
///    specification: string not null
///    product_at: date not null
///    production_batch: string not null
///    production_number: integer not null
///    batch_number: string not null
///    expire_at: date not null
#[derive(DeriveIden)]
enum ProcessingProduct {
    // 表名
    #[sea_orm(iden = "processing_product")]
    Table,
    // 主键
    #[sea_orm(iden = "product_id")]
    Id,
    // 唯一标识
    Uuid,
    // 加工产品名称
    Name,
    // 规格
    Specification,
    // 生产时间
    ProductAt,
    // 生产批次
    ProductionBatch,
    // 生产数量
    ProductionNumber,
    // 批号
    BatchNumber,
    // 有效期
    ExpireAt,
}

/// 原料库存管理表
/// 用于存储原料库存的基本信息
/// Table: processing_raw_material_inventory
/// Columns:
///    id: serial primary key not null
///    uuid: string unique not null
///    name: string not null
///    specification: string not null
///    quantity: integer not null
///    unit: string not null
///    purchase_at: date not null
#[derive(DeriveIden)]
enum ProcessingRawMaterialInventory {
    // 表名
    #[sea_orm(iden = "processing_raw_material_inventory")]
    Table,
    // 主键
    #[sea_orm(iden = "raw_material_inventory_id")]
    Id,
    // 唯一标识
    Uuid,
    // 原料名称
    Name,
    // 规格
    Specification,
    // 数量
    Quantity,
    // 单位
    Unit,
    // 采购时间
    PurchaseAt,
}

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}

/// 销售订单管理表
/// 用于存储销售订单的基本信息
/// Table: selling_order
/// Columns:
///     id: serial primary key not null
///     customerId: integer not null references customer(customer_id) on delete restrict
///     productId: integer not null references product(product_id) on delete restrict
///     qunatity: decimal(10, 2) not null
///     price: decimal(10, 2) not null
///     orderDate: date not null
#[derive(DeriveIden)]
enum SellingOrder {
    // 表
    #[sea_orm(iden = "selling_order")]
    Table,
    // 主键
    #[sea_orm(iden = "order_id")]
    Id,
    // 客户ID
    #[sea_orm(iden = "customer_id")]
    CustomerId,
    // 产品ID
    #[sea_orm(iden = "product_id")]
    ProductId,
    // 数量
    Quantity,
    // 价格
    Price,
    // 订单日期
    #[sea_orm(iden = "order_date")]
    OrderDate,
}

/// 客户信息表
/// 用于存储客户的基本信息
/// Table: selling_customer
/// Columns:
///     id: serial primary key not null
///     n
#[derive(DeriveIden)]
enum SellingCustomer {
    #[sea_orm(iden = "seliling_customer")]
    Table,
}

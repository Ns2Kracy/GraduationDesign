use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 快递员信息
        manager
            .create_table(
                Table::create()
                    .table(LogisticsCourier::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogisticsCourier::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LogisticsCourier::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(LogisticsCourier::Name).string().not_null())
                    .col(ColumnDef::new(LogisticsCourier::Gender).string().not_null())
                    .col(ColumnDef::new(LogisticsCourier::Phone).string().not_null())
                    .to_owned(),
            )
            .await?;

        // 收发件信息
        manager
            .create_table(
                Table::create()
                    .table(LogisticsExpress::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogisticsExpress::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::SenderName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::SenderPhone)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::SenderAddress)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::RecipientName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::RecipientPhone)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::RecipientAddress)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::ExpressCompany)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::ExpressNumber)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::ExpressStatus)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::ExpressAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::TrackingNumber)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsExpress::CourierId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_logistics_express_courier_id")
                    .from(LogisticsExpress::Table, LogisticsExpress::CourierId)
                    .to(LogisticsCourier::Table, LogisticsCourier::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 仓储管理
        manager
            .create_table(
                Table::create()
                    .table(LogisticsWarehouse::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogisticsWarehouse::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LogisticsWarehouse::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(LogisticsWarehouse::Name).string().not_null())
                    .col(
                        ColumnDef::new(LogisticsWarehouse::Address)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsWarehouse::Phone)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 运输记录
        manager
            .create_table(
                Table::create()
                    .table(LogisticsTransport::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LogisticsTransport::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LogisticsTransport::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(LogisticsTransport::ProductName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsTransport::TransportAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsTransport::TransportMethod)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsTransport::TransportStatus)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsTransport::Destination)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsTransport::ExpressId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LogisticsTransport::WarehouseId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_logistics_transport_express_id")
                    .from(LogisticsTransport::Table, LogisticsTransport::ExpressId)
                    .to(LogisticsExpress::Table, LogisticsExpress::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_logistics_transport_warehouse_id")
                    .from(LogisticsTransport::Table, LogisticsTransport::WarehouseId)
                    .to(LogisticsWarehouse::Table, LogisticsWarehouse::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 快递员信息
        manager
            .drop_table(Table::drop().table(LogisticsCourier::Table).to_owned())
            .await?;
        // 收发件信息
        manager
            .drop_table(Table::drop().table(LogisticsExpress::Table).to_owned())
            .await?;
        // 仓储管理
        manager
            .drop_table(Table::drop().table(LogisticsWarehouse::Table).to_owned())
            .await?;
        // 运输记录
        manager
            .drop_table(Table::drop().table(LogisticsTransport::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// 快递员信息表
/// 
/// 用于存储快递员的信息
/// 
/// Table: logistics_courier
/// 
/// Columns:
/// 
///    id: serial primary key not null
///    uuid: string unique not null
///    name: string not null
///    gender: string not null
///    phone: string not null
#[derive(DeriveIden)]
enum LogisticsCourier {
    #[sea_orm(iden = "logistics_courier")]
    Table,
    // 主键
    #[sea_orm(iden = "courier_id")]
    Id,
    // uuid
    Uuid,
    // 姓名
    Name,
    // 性别
    Gender,
    // 电话
    Phone,
}

/// 收发件信息表
/// 
/// 用于存储收发件的信息
/// 
/// Table: logistics_express
/// 
/// Columns:
/// 
///    id: interger serial primary key not null
///    uuid: string unique not null
///    sender_name: string not null
///    sender_phone: string not null
///    sender_address: string not null
///    recipient_name: string not null
///    recipient_phone: string not null
///    recipient_address: string not null
///    express_company: string not null
///    express_number: string not null
///    express_status: string not null
///    express_at: date not null
///    tracking_number: string not null
///    courier_id: interger not null references logistics_courier(id) on delete cascade on update cascade
#[derive(DeriveIden)]
enum LogisticsExpress {
    // 表名
    #[sea_orm(iden = "logistics_express")]
    Table,
    // 主键
    #[sea_orm(iden = "express_id")]
    Id,
    // uuid
    Uuid,
    // 发件人姓名
    SenderName,
    // 发件人电话
    SenderPhone,
    // 发件人地址
    SenderAddress,
    // 收件人姓名
    RecipientName,
    // 收件人电话
    RecipientPhone,
    // 收件人地址
    RecipientAddress,
    // 快递公司
    ExpressCompany,
    // 快递单号
    ExpressNumber,
    // 快递状态
    ExpressStatus,
    // 快递时间
    ExpressAt,
    // 追踪单号
    TrackingNumber,
    // 快递员id
    CourierId,
}

/// 仓储管理表
/// 
/// 用于存储仓储的信息
/// 
/// Table: logistics_warehouse
/// 
///    Columns:
///    id: serial primary key not null
///    uuid: string unique not null
///    name: string not null
///    address: string not null
///    phone: string not null
#[derive(DeriveIden)]
enum LogisticsWarehouse {
    // 表名
    #[sea_orm(iden = "logistics_warehouse")]
    Table,
    // 主键
    #[sea_orm(iden = "warehouse_id")]
    Id,
    // uuid
    Uuid,
    // 仓储名称
    Name,
    // 仓储地址
    Address,
    // 仓储电话
    Phone,
}

/// 运输记录表
/// 
/// 用于存储运输记录的信息
/// 
/// Table: logistics_transport
/// 
/// Columns:
/// 
///    id: serial primary key not null
///    uuid: string unique not null
///    product_name: string not null
///    transport_at: date not null
///    transport_method: string not null
///    transport_status: string not null
///    destination: string not null
///    express_id: interger not null references logistics_express(id) on delete cascade on update cascade
///    warehouse_id: interger not null references logistics_warehouse(id) on delete cascade on update cascade
#[derive(DeriveIden)]
enum LogisticsTransport {
    // 表名
    #[sea_orm(iden = "logistics_transport")]
    Table,
    // 主键
    #[sea_orm(iden = "record_id")]
    Id,
    // uuid
    Uuid,
    // 产品名称
    ProductName,
    // 运输时间
    TransportAt,
    // 运输方式
    TransportMethod,
    // 运输状态
    TransportStatus,
    // 目的地
    Destination,
    // 快递id
    ExpressId,
    // 仓储id
    WarehouseId,
}

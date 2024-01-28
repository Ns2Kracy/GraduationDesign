use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 员工信息表
        manager
            .create_table(
                Table::create()
                    .table(Employee::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Employee::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Employee::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Employee::Name).string().not_null())
                    .col(ColumnDef::new(Employee::Age).integer().not_null())
                    .col(ColumnDef::new(Employee::Part).string().not_null())
                    .col(ColumnDef::new(Employee::Position).string().not_null())
                    .col(ColumnDef::new(Employee::Phone).string().not_null())
                    .to_owned(),
            )
            .await?;
        // 种子信息表
        manager
            .create_table(
                Table::create()
                    .table(Seed::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Seed::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Seed::Uuid).string().not_null().unique_key())
                    .col(ColumnDef::new(Seed::Name).string().not_null())
                    .col(ColumnDef::new(Seed::Type).string().not_null())
                    .col(ColumnDef::new(Seed::Number).integer().not_null())
                    .col(ColumnDef::new(Seed::Unit).string().not_null())
                    .col(ColumnDef::new(Seed::Price).decimal().not_null())
                    .col(ColumnDef::new(Seed::Description).string().not_null())
                    .col(ColumnDef::new(Seed::Image).string().not_null())
                    .col(ColumnDef::new(Seed::Source).string().not_null())
                    .col(ColumnDef::new(Seed::Supplier).string().not_null())
                    .col(ColumnDef::new(Seed::PurchaseDate).date().not_null())
                    .to_owned(),
            )
            .await?;

        // 农药使用信息表
        manager
            .create_table(
                Table::create()
                    .table(Pesticide::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pesticide::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Pesticide::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Pesticide::Crops).string().not_null())
                    .col(ColumnDef::new(Pesticide::Name).string().not_null())
                    .col(ColumnDef::new(Pesticide::Type).string().not_null())
                    .col(ColumnDef::new(Pesticide::Number).integer().not_null())
                    .col(ColumnDef::new(Pesticide::Unit).string().not_null())
                    .col(ColumnDef::new(Pesticide::Price).decimal().not_null())
                    .col(ColumnDef::new(Pesticide::Description).string().not_null())
                    .col(ColumnDef::new(Pesticide::Image).string().not_null())
                    .col(ColumnDef::new(Pesticide::Source).string().not_null())
                    .col(ColumnDef::new(Pesticide::Supplier).string().not_null())
                    .col(ColumnDef::new(Pesticide::ApplyAt).date().not_null())
                    .col(
                        ColumnDef::new(Pesticide::ApplicationMethod)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Pesticide::Quantity).decimal().not_null())
                    // should be foreign key reference to Employee::Id
                    .col(ColumnDef::new(Pesticide::EmployeeId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        // foreign key reference to Employee::Id
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_pesticide_employee_id")
                    .from(Pesticide::Table, Pesticide::EmployeeId)
                    .to(Employee::Table, Employee::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 采收信息表
        manager
            .create_table(
                Table::create()
                    .table(Harvest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Harvest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Harvest::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Harvest::HarvestAt).date().not_null())
                    .col(ColumnDef::new(Harvest::Quantity).integer().not_null())
                    .col(ColumnDef::new(Harvest::EmployeeId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        // foreign key reference to Employee::Id
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_harvest_employee_id")
                    .from(Harvest::Table, Harvest::EmployeeId)
                    .to(Employee::Table, Employee::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 种植计划管理表
        manager
            .create_table(
                Table::create()
                    .table(PlantationPlan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantationPlan::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlan::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(PlantationPlan::Name).string().not_null())
                    .col(ColumnDef::new(PlantationPlan::Type).string().not_null())
                    .col(
                        ColumnDef::new(PlantationPlan::Description)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlantationPlan::Crop).string().not_null())
                    .col(
                        ColumnDef::new(PlantationPlan::PlantationAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlan::PlantationArea)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlan::ExpectedYield)
                            .decimal()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 作物监控信息表
        manager
            .create_table(
                Table::create()
                    .table(CropMonitoring::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CropMonitoring::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CropMonitoring::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(CropMonitoring::Name).string().not_null())
                    .col(ColumnDef::new(CropMonitoring::Crop).string().not_null())
                    .col(
                        ColumnDef::new(CropMonitoring::SoilMoisture)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CropMonitoring::SoilTemperature)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CropMonitoring::AirHumidity)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CropMonitoring::AirTemperature)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CropMonitoring::LightIntensity)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CropMonitoring::CarbonDioxide)
                            .decimal()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CropMonitoring::MonitorAt).date().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 员工信息表
        manager
            .drop_table(Table::drop().table(Employee::Table).to_owned())
            .await?;
        // 种子信息表
        manager
            .drop_table(Table::drop().table(Seed::Table).to_owned())
            .await?;
        // 种子信息表
        manager
            .drop_table(Table::drop().table(Pesticide::Table).to_owned())
            .await?;
        // 种子信息表
        manager
            .drop_table(Table::drop().table(Harvest::Table).to_owned())
            .await?;
        // 种子信息表
        manager
            .drop_table(Table::drop().table(PlantationPlan::Table).to_owned())
            .await?;
        // 种子信息表
        manager
            .drop_table(Table::drop().table(CropMonitoring::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// 员工信息表
/// 用于存储员工的基本信息
/// Table: 表名
/// Id: 主键
/// Uuid: 唯一标识
/// Name: 姓名
/// Age: 年龄
/// Part: 部门
/// Position: 职位
/// Phone: 电话
#[derive(DeriveIden)]
enum Employee {
    Table,
    Id,
    Uuid,
    // 姓名
    Name,
    // 年龄
    Age,
    // 部门
    Part,
    // 职位
    Position,
    // 电话
    Phone,
}

/// 种子信息表
/// 用于存储种子的基本信息
/// Table: 表名
/// Id: 主键
/// Uuid: 唯一标识
/// Name: 种子名称
/// Type: 种子类型
/// Number: 种子数量
/// Unit: 种子单位
/// Price: 种子单价
/// Description: 种子描述
/// Image: 种子图片
/// Source: 种子来源
/// Supplier: 种子供应商
/// PurchaseDate: 种子采购日期
#[derive(DeriveIden)]
enum Seed {
    Table,
    Id,
    Uuid,
    Name,
    Type,
    Number,
    Unit,
    Price,
    Description,
    Image,
    Source,
    Supplier,
    PurchaseDate,
}

/// 农药使用信息表
/// 用于存储农药的使用信息
/// Table: 表名
/// Id: 主键
/// Uuid: 唯一标识
/// Crop: 使用农药的作物
/// Name: 农药名称
/// Type: 农药类型
/// Number: 农药数量
/// Unit: 农药单位
/// Price: 农药单价
/// Description: 农药描述
/// Image: 农药图片
/// Source: 农药来源
/// Supplier: 农药供应商
/// ApplyAt: 农药使用日期
/// ApplicationMethod: 农药使用方法
/// Quantity: 农药用水量
/// EmployeeId: 使用农药的员工 Id
#[derive(DeriveIden)]
enum Pesticide {
    Table,
    Id,
    Uuid,
    Crops,
    Name,
    Type,
    Number,
    Unit,
    Price,
    Description,
    Image,
    Source,
    Supplier,
    ApplyAt,
    ApplicationMethod,
    Quantity,
    EmployeeId,
}

/// 采收信息表
/// 用于存储采收的基本信息
/// Table: 表名
/// Id: 主键
/// Uuid: 唯一标识
/// HarvestAt: 采收日期
/// Quantity: 采收数量
/// EmployeeId: 采收员工 Id
#[derive(DeriveIden)]
enum Harvest {
    Table,
    Id,
    Uuid,
    HarvestAt,
    Quantity,
    EmployeeId,
}

/// 种植计划管理表
/// 用于存储种植计划的基本信息
/// Table: 表名
/// Id: 主键
/// Uuid: 唯一标识
/// Name: 种植计划名称
/// Type: 种植计划类型
/// Description: 种植计划描述
/// Crop: 种植计划作物
/// PlantationAt: 种植计划种植日期
/// PlantationArea: 种植计划种植面积
/// ExpectedYield: 种植计划预期产量
#[derive(DeriveIden)]
enum PlantationPlan {
    Table,
    Id,
    Uuid,
    Name,
    Type,
    Description,
    Crop,
    PlantationAt,
    PlantationArea,
    ExpectedYield,
}

/// 作物监控信息表
/// 用于存储作物监控的基本信息
/// Table: 表名
/// Id: 主键
/// Uuid: 唯一标识
/// Name: 作物监控名称
/// Crop: 监控作物
/// SoilMoisture: 土壤湿度
/// SoilTemperature: 土壤温度
/// AirHumidity: 空气湿度
/// AirTemperature: 空气温度
/// LightIntensity: 光照强度
/// CarbonDioxide: 二氧化碳浓度
/// MonitorAt: 监控日期
#[derive(DeriveIden)]
enum CropMonitoring {
    Table,
    Id,
    Uuid,
    Name,
    Crop,
    SoilMoisture,
    SoilTemperature,
    AirHumidity,
    AirTemperature,
    LightIntensity,
    CarbonDioxide,
    MonitorAt,
}

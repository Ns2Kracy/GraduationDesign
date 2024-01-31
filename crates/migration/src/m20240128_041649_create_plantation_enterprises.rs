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
                    .table(PlantationEmployee::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantationEmployee::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationEmployee::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(PlantationEmployee::Name).string().not_null())
                    .col(ColumnDef::new(PlantationEmployee::Age).integer().not_null())
                    .col(ColumnDef::new(PlantationEmployee::Sex).string().not_null())
                    .col(ColumnDef::new(PlantationEmployee::Part).string().not_null())
                    .col(
                        ColumnDef::new(PlantationEmployee::Position)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationEmployee::Phone)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        // 种子信息表
        manager
            .create_table(
                Table::create()
                    .table(PlantationSeed::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantationSeed::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationSeed::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(PlantationSeed::Name).string().not_null())
                    .col(ColumnDef::new(PlantationSeed::Type).string().not_null())
                    .col(ColumnDef::new(PlantationSeed::Number).integer().not_null())
                    .col(ColumnDef::new(PlantationSeed::Unit).string().not_null())
                    .col(ColumnDef::new(PlantationSeed::Price).decimal().not_null())
                    .col(
                        ColumnDef::new(PlantationSeed::Description)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlantationSeed::Source).string().not_null())
                    .col(ColumnDef::new(PlantationSeed::Supplier).string().not_null())
                    .col(
                        ColumnDef::new(PlantationSeed::PurchaseDate)
                            .date()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 农药使用信息表
        manager
            .create_table(
                Table::create()
                    .table(PlantationPesticideUse::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Crops)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Type)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Number)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Unit)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Price)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Source)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Supplier)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::ApplyAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::ApplicationMethod)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::Quantity)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPesticideUse::EmployeeId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        // foreign key reference to Employee::Id
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_pesticide_employee_id")
                    .from(
                        PlantationPesticideUse::Table,
                        PlantationPesticideUse::EmployeeId,
                    )
                    .to(PlantationEmployee::Table, PlantationEmployee::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 采收信息表
        manager
            .create_table(
                Table::create()
                    .table(PlantationHarvest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantationHarvest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationHarvest::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationHarvest::HarvestAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationHarvest::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationHarvest::EmployeeId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        // foreign key reference to Employee::Id
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_harvest_employee_id")
                    .from(PlantationHarvest::Table, PlantationHarvest::EmployeeId)
                    .to(PlantationEmployee::Table, PlantationEmployee::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 种植计划管理表
        manager
            .create_table(
                Table::create()
                    .table(PlantationPlantationPlan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::Type)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::Crop)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::PlantationAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::PlantationArea)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationPlantationPlan::ExpectedYield)
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
                    .table(PlantationCropMonitoring::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::Crop)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::SoilMoisture)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::SoilTemperature)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::AirHumidity)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::AirTemperature)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::LightIntensity)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::CarbonDioxide)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantationCropMonitoring::MonitorAt)
                            .date()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 员工信息表
        manager
            .drop_table(Table::drop().table(PlantationEmployee::Table).to_owned())
            .await?;
        // 种子信息表
        manager
            .drop_table(Table::drop().table(PlantationSeed::Table).to_owned())
            .await?;
        // 种子信息表
        manager
            .drop_table(
                Table::drop()
                    .table(PlantationPesticideUse::Table)
                    .to_owned(),
            )
            .await?;
        // 种子信息表
        manager
            .drop_table(Table::drop().table(PlantationHarvest::Table).to_owned())
            .await?;
        // 种子信息表
        manager
            .drop_table(
                Table::drop()
                    .table(PlantationPlantationPlan::Table)
                    .to_owned(),
            )
            .await?;
        // 种子信息表
        manager
            .drop_table(
                Table::drop()
                    .table(PlantationCropMonitoring::Table)
                    .to_owned(),
            )
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
/// Sex: 性别
/// Part: 部门
/// Position: 职位
/// Phone: 电话
#[derive(DeriveIden)]
enum PlantationEmployee {
    #[sea_orm(iden = "processing_employee")]
    Table,
    Id,
    Uuid,
    Name,
    Age,
    Sex,
    Part,
    Position,
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
/// Source: 种子来源
/// Supplier: 种子供应商
/// PurchaseDate: 种子采购日期
#[derive(DeriveIden)]
enum PlantationSeed {
    #[sea_orm(iden = "processingeed")]
    Table,
    Id,
    Uuid,
    Name,
    Type,
    Number,
    Unit,
    Price,
    Description,
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
/// Source: 农药来源
/// Supplier: 农药供应商
/// ApplyAt: 农药使用日期
/// ApplicationMethod: 农药使用方法
/// Quantity: 农药用水量
/// EmployeeId: 使用农药的员工 Id
#[derive(DeriveIden)]
enum PlantationPesticideUse {
    #[sea_orm(iden = "processing_pesticide_use")]
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
enum PlantationHarvest {
    #[sea_orm(iden = "processing_harvest")]
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
enum PlantationPlantationPlan {
    #[sea_orm(iden = "processing_plantation_plan")]
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
enum PlantationCropMonitoring {
    #[sea_orm(iden = "processing_crop_monitoring")]
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

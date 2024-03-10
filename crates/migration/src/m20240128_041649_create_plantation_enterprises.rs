use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 员工信息
        manager
            .create_table(
                Table::create()
                    .table(PlantingEmployee::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantingEmployee::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingEmployee::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(PlantingEmployee::Name).string().not_null())
                    .col(ColumnDef::new(PlantingEmployee::Age).integer().not_null())
                    .col(ColumnDef::new(PlantingEmployee::Sex).string().not_null())
                    .col(ColumnDef::new(PlantingEmployee::Part).string().not_null())
                    .col(
                        ColumnDef::new(PlantingEmployee::Position)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlantingEmployee::Phone).string().not_null())
                    .to_owned(),
            )
            .await?;

        // 种子信息
        manager
            .create_table(
                Table::create()
                    .table(PlantingSeed::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantingSeed::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingSeed::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(PlantingSeed::Name).string().not_null())
                    .col(ColumnDef::new(PlantingSeed::Type).string().not_null())
                    .col(ColumnDef::new(PlantingSeed::Number).integer().not_null())
                    .col(ColumnDef::new(PlantingSeed::Unit).string().not_null())
                    .col(ColumnDef::new(PlantingSeed::Price).decimal().not_null())
                    .col(
                        ColumnDef::new(PlantingSeed::Description)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlantingSeed::Source).string().not_null())
                    .col(ColumnDef::new(PlantingSeed::Supplier).string().not_null())
                    .col(ColumnDef::new(PlantingSeed::PurchaseDate).date().not_null())
                    .to_owned(),
            )
            .await?;

        // 农药使用信息
        manager
            .create_table(
                Table::create()
                    .table(PlantingPesticideUse::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Crops)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Type)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Number)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Unit)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Price)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Source)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Supplier)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::ApplyAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::ApplicationMethod)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::Quantity)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPesticideUse::EmployeeId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("fk_pesticide_employee_id")
                    .from(
                        PlantingPesticideUse::Table,
                        PlantingPesticideUse::EmployeeId,
                    )
                    .to(PlantingEmployee::Table, PlantingEmployee::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 采收信息
        manager
            .create_table(
                Table::create()
                    .table(PlantingHarvest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantingHarvest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingHarvest::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(PlantingHarvest::Crop).string().not_null())
                    .col(ColumnDef::new(PlantingHarvest::HarvestAt).date().not_null())
                    .col(
                        ColumnDef::new(PlantingHarvest::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingHarvest::EmployeeId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_harvest_employee_id")
                    .from(PlantingHarvest::Table, PlantingHarvest::EmployeeId)
                    .to(PlantingEmployee::Table, PlantingEmployee::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 种植计划管理
        manager
            .create_table(
                Table::create()
                    .table(PlantingPlantingPlan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::Type)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::Crop)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::PlantingAt)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::PlantingArea)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingPlantingPlan::ExpectedYield)
                            .decimal()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 作物监控信息
        manager
            .create_table(
                Table::create()
                    .table(PlantingCropMonitoring::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::Uuid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::Crop)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::SoilMoisture)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::SoilTemperature)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::AirHumidity)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::AirTemperature)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::LightIntensity)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::CarbonDioxide)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlantingCropMonitoring::MonitorAt)
                            .date()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 员工信息
        manager
            .drop_table(Table::drop().table(PlantingEmployee::Table).to_owned())
            .await?;
        // 种子信息
        manager
            .drop_table(Table::drop().table(PlantingSeed::Table).to_owned())
            .await?;
        // 种子信息
        manager
            .drop_table(Table::drop().table(PlantingPesticideUse::Table).to_owned())
            .await?;
        // 种子信息
        manager
            .drop_table(Table::drop().table(PlantingHarvest::Table).to_owned())
            .await?;
        // 种子信息
        manager
            .drop_table(Table::drop().table(PlantingPlantingPlan::Table).to_owned())
            .await?;
        // 种子信息
        manager
            .drop_table(
                Table::drop()
                    .table(PlantingCropMonitoring::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// 员工信息表
///
/// 用于存储员工的基本信息
///
/// Table: plant_employee
///
/// Columns:
///
///    Id: interger serial primary key not null
///    Uuid: string unique key not null
///    Name: string not null
///    Age: integer not null
///    Sex: string not null
///    Part: string not null
///    Position: string not null
///    Phone: string not null
#[derive(DeriveIden)]
enum PlantingEmployee {
    // 表名
    #[sea_orm(iden = "planting_employee")]
    Table,
    // 主键
    #[sea_orm(iden = "employee_id")]
    Id,
    // 唯一标识
    Uuid,
    // 姓名
    Name,
    // 年龄
    Age,
    // 性别
    Sex,
    // 部门
    Part,
    // 职位
    Position,
    // 电话
    Phone,
}

/// 种子信息表
///
/// 用于存储种子的基本信息
///
/// Table: plant_seed
///
/// Columns:
///
///    id: serial primary key not null
///    uuid: string unique key not null
///    name: string not null
///    type: string not null
///    number: integer not null
///    unit: string not null
///    price: decimal not null
///    description: string not null
///    source: string not null
///    supplier: string not null
///    purchaseDate: date not null
#[derive(DeriveIden)]
enum PlantingSeed {
    // 表名
    #[sea_orm(iden = "planting_seed")]
    Table,
    // 主键
    #[sea_orm(iden = "seed_id")]
    Id,
    // 唯一标识
    Uuid,
    // 种子名称
    Name,
    // 种子类型
    Type,
    // 种子数量
    Number,
    // 种子单位
    Unit,
    // 种子单价
    Price,
    // 种子描述
    Description,
    // 种子来源
    Source,
    // 种子供应商
    Supplier,
    // 种子采购日期
    PurchaseDate,
}

/// 农药使用信息表
///
/// 用于存储农药的使用信息
///
/// Table: plant_pesticide_use
///
/// Columns:
///
///    id: serial primary key not null
///    uuid: string unique key not null
///    crops: string not null
///    name: string not null
///    type: string not null
///    number: integer not null
///    unit: string not null
///    price: decimal not null
///    description: string not null
///    source: string not null
///    supplier: string not null
///    applyAt: date not null
///    applicationMethod: string not null
///    quantity: decimal not null
///    employeeId: integer not null references plant_employee(employee_id) on delete cascade on update cascade
#[derive(DeriveIden)]
enum PlantingPesticideUse {
    // 表名
    #[sea_orm(iden = "planting_pesticide_use")]
    Table,
    // 主键
    #[sea_orm(iden = "pesticide_use_id")]
    Id,
    // 唯一标识
    Uuid,
    // 使用农药的作物
    Crops,
    // 农药名称
    Name,
    // 农药类型
    Type,
    // 农药数量
    Number,
    // 农药单位
    Unit,
    // 农药单价
    Price,
    // 农药描述
    Description,
    // 农药来源
    Source,
    // 农药供应商
    Supplier,
    // 农药使用日期
    ApplyAt,
    // 农药使用方法
    ApplicationMethod,
    // 农药用水量
    Quantity,
    // 使用农药的员工 Id
    EmployeeId,
}

/// 采收信息表
///
/// 用于存储采收的基本信息
///
/// Table: plant_harvest
///
/// Columns:
///
///    id: serial primary key not null
///    uuid: string unique key not null
///    crop: string not null
///    harvest_at: date not null
///    quantity: integer not null
///    employee_id: integer not null
#[derive(DeriveIden)]
enum PlantingHarvest {
    // 表名
    #[sea_orm(iden = "planting_harvest")]
    Table,
    // 主键
    #[sea_orm(iden = "harvest_id")]
    Id,
    // 唯一标识
    Uuid,
    // 采收作物
    Crop,
    // 采收日期
    #[sea_orm(iden = "harvest_at")]
    HarvestAt,
    // 采收数量
    Quantity,
    // 采收员工 Id
    #[sea_orm(iden = "employee_id")]
    EmployeeId,
}

/// 种植计划管理表
///
/// 用于存储种植计划的基本信息
///
/// Table: plant_planting_plan
///
/// Columns:
///
///    id: interger serial primary key not null
///    uuid: string unique key not null
///    name: string not null
///    type: string not null
///    description: string not null
///    crop: string not null
///    planting_at: date not null
///    planting_area: decimal not null
///    expected_yield: decimal not null
#[derive(DeriveIden)]
enum PlantingPlantingPlan {
    // 表名
    #[sea_orm(iden = "planting_Planting_plan")]
    Table,
    // 主键
    #[sea_orm(iden = "planting_plan_id")]
    Id,
    // 唯一标识
    Uuid,
    // 种植计划名称
    Name,
    // 种植计划类型
    Type,
    // 种植计划描述
    Description,
    // 种植计划作物
    Crop,
    // 种植计划种植日期
    #[sea_orm(iden = "planting_at")]
    PlantingAt,
    // 种植计划种植面积
    #[sea_orm(iden = "planting_area")]
    PlantingArea,
    // 种植计划预期产量
    #[sea_orm(iden = "expected_yield")]
    ExpectedYield,
}

/// 作物监控信息表
///
/// 用于存储作物监控的基本信息
///
/// Table: plant_crop_monitoring
///
/// Columns:
///
///    id: serial primary key not null
///    uuid: string unique key not null
///    name: string not null
///    crop: string not null
///    soil_moisture: interger not null
///    soil_temperature: interger not null
///    air_humidity: interger not null
///    air_temperature: interger not null
///    light_intensity: interger not null
///    carbon_dioxide: interger not null
///    monitor_at: date not null
#[derive(DeriveIden)]
enum PlantingCropMonitoring {
    // 表名
    #[sea_orm(iden = "planting_crop_monitoring")]
    Table,
    // 主键
    #[sea_orm(iden = "crop_monitoring_id")]
    Id,
    // 唯一标识
    Uuid,
    // 作物监控名称
    Name,
    // 监控作物
    Crop,
    // 土壤湿度
    #[sea_orm(iden = "soil_moisture")]
    SoilMoisture,
    // 土壤温度
    #[sea_orm(iden = "soil_temperature")]
    SoilTemperature,
    // 空气湿度
    #[sea_orm(iden = "air_humidity")]
    AirHumidity,
    // 空气温度
    #[sea_orm(iden = "air_temperature")]
    AirTemperature,
    // 光照强度
    #[sea_orm(iden = "light_intensity")]
    LightIntensity,
    // 二氧化碳浓度
    #[sea_orm(iden = "carbon_dioxide")]
    CarbonDioxide,
    // 监控日期
    #[sea_orm(iden = "monitor_at")]
    MonitorAt,
}

#![allow(clippy::enum_variant_names)]
#![allow(unused)]

pub use sea_orm_migration::prelude::*;

pub mod m20240321_163618_create_farms_table;
pub mod m20240321_163642_create_seeds_table;
pub mod m20240321_164456_create_planting_records_table;
pub mod m20240321_164516_create_pesticide_usage_records_table;
pub mod m20240321_164532_create_harvest_records_table;
pub mod m20240321_164553_create_product_inspection_records_table;
pub mod m20240321_164619_create_pesticide_residue_detection_records_table;
pub mod m20240321_165622_create_risk_assessment_table;
pub mod m20240321_165638_create_alert_information_table;
pub mod m20240321_165646_create_report_table;
pub mod m20240321_165654_create_analysis_table;
pub mod m20240321_165701_create_users_table;
pub mod m20240321_165712_create_operation_logs_table;
pub mod m20240321_165725_create_enforcement_records_table;
pub mod m20240321_165743_create_penalty_records_table;
pub mod m20240321_165753_create_statistical_data_table;
pub mod m20240321_165803_create_query_logs_table;
pub mod m20240321_165814_create_quality_feedback_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240321_163618_create_farms_table::Migration),
            Box::new(m20240321_163642_create_seeds_table::Migration),
            Box::new(m20240321_164456_create_planting_records_table::Migration),
            Box::new(m20240321_164516_create_pesticide_usage_records_table::Migration),
            Box::new(m20240321_164532_create_harvest_records_table::Migration),
            Box::new(m20240321_164553_create_product_inspection_records_table::Migration),
            Box::new(m20240321_164619_create_pesticide_residue_detection_records_table::Migration),
            Box::new(m20240321_165622_create_risk_assessment_table::Migration),
            Box::new(m20240321_165638_create_alert_information_table::Migration),
            Box::new(m20240321_165646_create_report_table::Migration),
            Box::new(m20240321_165654_create_analysis_table::Migration),
            Box::new(m20240321_165701_create_users_table::Migration),
            Box::new(m20240321_165712_create_operation_logs_table::Migration),
            Box::new(m20240321_165725_create_enforcement_records_table::Migration),
            Box::new(m20240321_165743_create_penalty_records_table::Migration),
            Box::new(m20240321_165753_create_statistical_data_table::Migration),
            Box::new(m20240321_165803_create_query_logs_table::Migration),
            Box::new(m20240321_165814_create_quality_feedback_table::Migration),
        ]
    }
}

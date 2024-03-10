#![allow(clippy::enum_variant_names)]

pub use sea_orm_migration::prelude::*;

mod m20240128_041649_create_plantation_enterprises;
mod m20240128_064315_create_processing_enterprises;
mod m20240131_085823_create_logistics_enterprises;
mod m20240212_130145_create_selling_enterprises;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240128_041649_create_plantation_enterprises::Migration),
            Box::new(m20240128_064315_create_processing_enterprises::Migration),
            Box::new(m20240131_085823_create_logistics_enterprises::Migration),
            // Box::new(m20240212_130145_create_selling_enterprises::Migration),
        ]
    }
}

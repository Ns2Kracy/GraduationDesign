//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "farms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub farm_id: i32,
    pub farm_name: String,
    pub location: String,
    pub area: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::enforcement_records::Entity")]
    EnforcementRecords,
    #[sea_orm(has_many = "super::harvest_records::Entity")]
    HarvestRecords,
    #[sea_orm(has_many = "super::penalty_records::Entity")]
    PenaltyRecords,
    #[sea_orm(has_many = "super::pesticide_residue_detection_records::Entity")]
    PesticideResidueDetectionRecords,
    #[sea_orm(has_many = "super::pesticide_usage_records::Entity")]
    PesticideUsageRecords,
    #[sea_orm(has_many = "super::planting_records::Entity")]
    PlantingRecords,
    #[sea_orm(has_many = "super::product_inspection_records::Entity")]
    ProductInspectionRecords,
    #[sea_orm(has_many = "super::quality_feedback::Entity")]
    QualityFeedback,
}

impl Related<super::enforcement_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EnforcementRecords.def()
    }
}

impl Related<super::harvest_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HarvestRecords.def()
    }
}

impl Related<super::penalty_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PenaltyRecords.def()
    }
}

impl Related<super::pesticide_residue_detection_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PesticideResidueDetectionRecords.def()
    }
}

impl Related<super::pesticide_usage_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PesticideUsageRecords.def()
    }
}

impl Related<super::planting_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlantingRecords.def()
    }
}

impl Related<super::product_inspection_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductInspectionRecords.def()
    }
}

impl Related<super::quality_feedback::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QualityFeedback.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
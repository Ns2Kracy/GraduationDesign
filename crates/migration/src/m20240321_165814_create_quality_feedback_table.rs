use sea_orm_migration::prelude::*;

use crate::m20240321_163618_create_farms_table::Farms;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(QualityFeedback::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QualityFeedback::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QualityFeedback::FarmId).integer().not_null())
                    .col(
                        ColumnDef::new(QualityFeedback::FeedbackDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QualityFeedback::FeedbackContent)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QualityFeedback::FeedbackStatus)
                            .string_len(50)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_quality_feedback_farm_id")
                            .from(QualityFeedback::Table, QualityFeedback::FarmId)
                            .to(Farms::Table, Farms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QualityFeedback::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum QualityFeedback {
    #[sea_orm(iden = "quality_feedback")]
    Table,
    #[sea_orm(iden = "feedback_id")]
    Id,
    #[sea_orm(iden = "FarmId")]
    FarmId,
    #[sea_orm(iden = "feedback_date")]
    FeedbackDate,
    #[sea_orm(iden = "feedback_content")]
    FeedbackContent,
    #[sea_orm(iden = "feedback_status")]
    FeedbackStatus,
}

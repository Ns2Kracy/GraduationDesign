use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string_len(100).not_null())
                    .col(ColumnDef::new(Users::Password).string_len(100).not_null())
                    .col(ColumnDef::new(Users::Role).string_len(50).not_null())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Users::DeletedAt).timestamp().null())
                    .col(ColumnDef::new(Users::LastLoginAt).timestamp().null())
                    .col(ColumnDef::new(Users::Status).string_len(20).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    #[sea_orm(iden = "users")]
    Table,
    #[sea_orm(iden = "user_id")]
    Id,
    #[sea_orm(iden = "username")]
    Username,
    #[sea_orm(iden = "password")]
    Password,
    #[sea_orm(iden = "role")]
    Role,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
    #[sea_orm(iden = "deleted_at")]
    DeletedAt,
    #[sea_orm(iden = "last_login_at")]
    LastLoginAt,
    #[sea_orm(iden = "status")]
    Status,
}

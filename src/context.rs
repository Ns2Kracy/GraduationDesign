use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: DatabaseConnection,
}

pub type Ctx = Arc<Context>;

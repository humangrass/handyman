use std::sync::Arc;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Repository {
    pub(crate) database: Arc<PgPool>,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { database: pool.into() }
    }
    pub fn pool(&self) -> &PgPool {
        &self.database
    }
}

mod config;

use crate::util::tx_manager::config::Config;
use sqlx::{PgConnection, PgPool};

pub struct TxManager {
    pool: PgPool,
}

impl TxManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let config = Config::new();
        Ok(Self {
            pool: PgPool::connect(&config.database_url).await?,
        })
    }

    pub async fn execute<T, F>(&self, f: F) -> Result<T, sqlx::Error>
    where
        F: AsyncFnOnce(&mut PgConnection) -> Result<T, sqlx::Error>,
    {
        let mut tx = self.pool.begin().await?;

        match f(&mut tx).await {
            Ok(v) => {
                tx.commit().await?;
                Ok(v)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}

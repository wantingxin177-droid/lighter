use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub mod models;
pub mod queries;

pub use models::*;
pub use queries::*;

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .min_connections(10)
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(database_url)
            .await?;

        // 运行迁移
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

// Redis缓存包装器
#[derive(Clone)]
pub struct RedisCache {
    client: redis::Client,
    connection_manager: redis::aio::ConnectionManager,
}

impl RedisCache {
    pub async fn new(redis_url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let connection_manager = redis::aio::ConnectionManager::new(client.clone()).await?;
        
        Ok(Self {
            client,
            connection_manager,
        })
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> anyhow::Result<Option<T>> {
        let mut conn = self.connection_manager.clone();
        let value: Option<Vec<u8>> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await?;
        
        match value {
            Some(v) => {
                let deserialized = rmp_serde::from_slice(&v)?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    pub async fn set<T: serde::Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: u64,
    ) -> anyhow::Result<()> {
        let mut conn = self.connection_manager.clone();
        let serialized = rmp_serde::to_vec(value)?;
        
        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_seconds)
            .arg(serialized)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> anyhow::Result<()> {
        let mut conn = self.connection_manager.clone();
        redis::cmd("DEL")
            .arg(key)
            .query_async::<_, ()>(&mut conn)
            .await?;
        Ok(())
    }

    pub async fn publish(&self, channel: &str, message: &str) -> anyhow::Result<()> {
        let mut conn = self.connection_manager.clone();
        redis::cmd("PUBLISH")
            .arg(channel)
            .arg(message)
            .query_async::<_, ()>(&mut conn)
            .await?;
        Ok(())
    }
}

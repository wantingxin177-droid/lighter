use crate::db::{models::*, Database, RedisCache};
use moka::future::Cache;
use std::sync::Arc;

pub struct BlockService {
    db: Arc<Database>,
    cache: Arc<RedisCache>,
    memory_cache: Cache<String, Block>,
}

impl BlockService {
    pub fn new(
        db: Arc<Database>,
        cache: Arc<RedisCache>,
        memory_cache: Cache<String, Block>,
    ) -> Self {
        Self {
            db,
            cache,
            memory_cache,
        }
    }

    pub async fn get_latest_block(&self) -> anyhow::Result<Option<Block>> {
        // 1. 检查内存缓存
        let cache_key = "block:latest".to_string();
        if let Some(block) = self.memory_cache.get(&cache_key).await {
            return Ok(Some(block));
        }

        // 2. 检查Redis缓存
        if let Some(block) = self.cache.get::<Block>("block:latest").await? {
            self.memory_cache.insert(cache_key, block.clone()).await;
            return Ok(Some(block));
        }

        // 3. 查询数据库
        let block = self.db.get_latest_block().await?;
        
        if let Some(ref b) = block {
            // 写入多级缓存
            self.cache.set("block:latest", b, 5).await?;
            self.memory_cache.insert(cache_key, b.clone()).await;
        }

        Ok(block)
    }

    pub async fn get_block_by_height(&self, height: i64) -> anyhow::Result<Option<Block>> {
        let cache_key = format!("block:{}", height);

        // 1. 检查内存缓存
        if let Some(block) = self.memory_cache.get(&cache_key).await {
            return Ok(Some(block));
        }

        // 2. 检查Redis缓存
        if let Some(block) = self.cache.get::<Block>(&cache_key).await? {
            self.memory_cache.insert(cache_key, block.clone()).await;
            return Ok(Some(block));
        }

        // 3. 查询数据库
        let block = self.db.get_block_by_height(height).await?;

        if let Some(ref b) = block {
            // 写入多级缓存
            self.cache.set(&cache_key, b, 300).await?;
            self.memory_cache.insert(cache_key, b.clone()).await;
        }

        Ok(block)
    }

    pub async fn get_blocks_range(&self, start: i64, end: i64) -> anyhow::Result<Vec<Block>> {
        // 范围查询通常不适合缓存，直接查数据库
        self.db.get_blocks_range(start, end).await
    }
}

use super::models::*;
use super::Database;
use anyhow::Result;
use sqlx::Row;

impl Database {
    // ========== 区块查询 ==========
    
    pub async fn get_latest_block(&self) -> Result<Option<Block>> {
        let block = sqlx::query_as::<_, Block>(
            r#"
            SELECT height, hash, prev_hash, timestamp, tx_count, l1_batch_number, created_at
            FROM blocks
            ORDER BY height DESC
            LIMIT 1
            "#
        )
        .fetch_optional(self.pool())
        .await?;
        
        Ok(block)
    }

    pub async fn get_block_by_height(&self, height: i64) -> Result<Option<Block>> {
        let block = sqlx::query_as::<_, Block>(
            r#"
            SELECT height, hash, prev_hash, timestamp, tx_count, l1_batch_number, created_at
            FROM blocks
            WHERE height = $1
            "#
        )
        .bind(height)
        .fetch_optional(self.pool())
        .await?;
        
        Ok(block)
    }

    pub async fn get_blocks_range(&self, start: i64, end: i64) -> Result<Vec<Block>> {
        let blocks = sqlx::query_as::<_, Block>(
            r#"
            SELECT height, hash, prev_hash, timestamp, tx_count, l1_batch_number, created_at
            FROM blocks
            WHERE height >= $1 AND height <= $2
            ORDER BY height DESC
            LIMIT 100
            "#
        )
        .bind(start)
        .bind(end)
        .fetch_all(self.pool())
        .await?;
        
        Ok(blocks)
    }

    pub async fn insert_block(&self, block: &Block) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO blocks (height, hash, prev_hash, timestamp, tx_count, l1_batch_number)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (height) DO NOTHING
            "#
        )
        .bind(block.height)
        .bind(&block.hash)
        .bind(&block.prev_hash)
        .bind(block.timestamp)
        .bind(block.tx_count)
        .bind(block.l1_batch_number)
        .execute(self.pool())
        .await?;
        
        Ok(())
    }

    // ========== 交易查询 ==========
    
    pub async fn get_transaction(&self, hash: &str) -> Result<Option<Transaction>> {
        let tx = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT hash, block_height, from_addr, to_addr, value, gas_price, gas_used, status, timestamp, created_at
            FROM transactions
            WHERE hash = $1
            "#
        )
        .bind(hash)
        .fetch_optional(self.pool())
        .await?;
        
        Ok(tx)
    }

    pub async fn get_block_transactions(&self, block_height: i64) -> Result<Vec<Transaction>> {
        let txs = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT hash, block_height, from_addr, to_addr, value, gas_price, gas_used, status, timestamp, created_at
            FROM transactions
            WHERE block_height = $1
            ORDER BY timestamp DESC
            "#
        )
        .bind(block_height)
        .fetch_all(self.pool())
        .await?;
        
        Ok(txs)
    }

    pub async fn get_recent_transactions(&self, limit: i64) -> Result<Vec<Transaction>> {
        let txs = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT hash, block_height, from_addr, to_addr, value, gas_price, gas_used, status, timestamp, created_at
            FROM transactions
            ORDER BY timestamp DESC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(self.pool())
        .await?;
        
        Ok(txs)
    }

    pub async fn insert_transaction(&self, tx: &Transaction) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO transactions (hash, block_height, from_addr, to_addr, value, gas_price, gas_used, status, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (hash) DO NOTHING
            "#
        )
        .bind(&tx.hash)
        .bind(tx.block_height)
        .bind(&tx.from_addr)
        .bind(&tx.to_addr)
        .bind(&tx.value)
        .bind(&tx.gas_price)
        .bind(tx.gas_used)
        .bind(&tx.status)
        .bind(tx.timestamp)
        .execute(self.pool())
        .await?;
        
        Ok(())
    }

    // ========== 统计查询 ==========
    
    pub async fn get_stats(&self) -> Result<StatsOverview> {
        let row = sqlx::query(
            r#"
            SELECT 
                (SELECT COUNT(*) FROM blocks) as total_blocks,
                (SELECT COUNT(*) FROM transactions) as total_transactions,
                (SELECT MAX(height) FROM blocks) as latest_height,
                NOW() as last_sync_time
            "#
        )
        .fetch_one(self.pool())
        .await?;

        Ok(StatsOverview {
            total_blocks: row.get("total_blocks"),
            total_transactions: row.get("total_transactions"),
            latest_height: row.get::<Option<i64>, _>("latest_height").unwrap_or(0),
            ws_connected: true, // 由外部更新
            last_sync_time: row.get("last_sync_time"),
        })
    }

    // ========== 市场数据查询 ==========
    
    pub async fn get_recent_trades(&self, symbol: &str, limit: i64) -> Result<Vec<Trade>> {
        let trades = sqlx::query_as::<_, Trade>(
            r#"
            SELECT id, symbol, price, amount, side, timestamp
            FROM trades
            WHERE symbol = $1
            ORDER BY timestamp DESC
            LIMIT $2
            "#
        )
        .bind(symbol)
        .bind(limit)
        .fetch_all(self.pool())
        .await?;
        
        Ok(trades)
    }

    pub async fn insert_trade(&self, trade: &Trade) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO trades (symbol, price, amount, side, timestamp)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(&trade.symbol)
        .bind(&trade.price)
        .bind(&trade.amount)
        .bind(&trade.side)
        .bind(trade.timestamp)
        .execute(self.pool())
        .await?;
        
        Ok(())
    }
}

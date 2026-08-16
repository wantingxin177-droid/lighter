-- 区块表
CREATE TABLE IF NOT EXISTS blocks (
    height BIGINT PRIMARY KEY,
    hash VARCHAR(66) NOT NULL UNIQUE,
    prev_hash VARCHAR(66) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    tx_count INTEGER NOT NULL DEFAULT 0,
    l1_batch_number BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 区块索引
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_blocks_l1_batch ON blocks(l1_batch_number);

-- 交易表
CREATE TABLE IF NOT EXISTS transactions (
    hash VARCHAR(66) PRIMARY KEY,
    block_height BIGINT NOT NULL REFERENCES blocks(height),
    from_addr VARCHAR(42) NOT NULL,
    to_addr VARCHAR(42) NOT NULL,
    value NUMERIC(78, 0) NOT NULL DEFAULT 0,
    gas_price NUMERIC(78, 0) NOT NULL DEFAULT 0,
    gas_used BIGINT NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 交易索引
CREATE INDEX IF NOT EXISTS idx_txs_block ON transactions(block_height);
CREATE INDEX IF NOT EXISTS idx_txs_from ON transactions(from_addr);
CREATE INDEX IF NOT EXISTS idx_txs_to ON transactions(to_addr);
CREATE INDEX IF NOT EXISTS idx_txs_timestamp ON transactions(timestamp DESC);

-- 市场数据表
CREATE TABLE IF NOT EXISTS market_data (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    price NUMERIC(36, 18) NOT NULL,
    volume_24h NUMERIC(36, 18) NOT NULL DEFAULT 0,
    change_24h NUMERIC(10, 4) NOT NULL DEFAULT 0,
    high_24h NUMERIC(36, 18) NOT NULL DEFAULT 0,
    low_24h NUMERIC(36, 18) NOT NULL DEFAULT 0,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(symbol, timestamp)
);

CREATE INDEX IF NOT EXISTS idx_market_symbol ON market_data(symbol, timestamp DESC);

-- 成交记录表
CREATE TABLE IF NOT EXISTS trades (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    price NUMERIC(36, 18) NOT NULL,
    amount NUMERIC(36, 18) NOT NULL,
    side VARCHAR(10) NOT NULL CHECK (side IN ('buy', 'sell')),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_trades_symbol ON trades(symbol, timestamp DESC);

-- 资金费率表
CREATE TABLE IF NOT EXISTS funding_rates (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    rate NUMERIC(10, 8) NOT NULL,
    next_funding_time TIMESTAMPTZ NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(symbol, timestamp)
);

CREATE INDEX IF NOT EXISTS idx_funding_symbol ON funding_rates(symbol, timestamp DESC);

-- L1 批次表
CREATE TABLE IF NOT EXISTS l1_batches (
    batch_number BIGINT PRIMARY KEY,
    l1_tx_hash VARCHAR(66) NOT NULL,
    block_range_start BIGINT NOT NULL,
    block_range_end BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending'
);

CREATE INDEX IF NOT EXISTS idx_l1_batches_time ON l1_batches(timestamp DESC);

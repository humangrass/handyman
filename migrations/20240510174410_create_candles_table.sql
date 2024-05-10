-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE candles
(
    uuid               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    exchange           VARCHAR(20)      NOT NULL,
    symbol             VARCHAR(20)      NOT NULL,
    timeframe          VARCHAR(10)      NOT NULL,
    open_time          TIMESTAMP        NOT NULL,
    open               DOUBLE PRECISION NOT NULL,
    high               DOUBLE PRECISION,
    low                DOUBLE PRECISION,
    close              DOUBLE PRECISION,
    asset_volume       DOUBLE PRECISION,
    quote_asset_volume DOUBLE PRECISION,
    usdt_volume        DOUBLE PRECISION
);

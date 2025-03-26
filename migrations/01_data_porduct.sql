-- Your SQL goes here
DROP TABLE IF EXISTS "data_products";
CREATE TABLE data_products (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL,
    status VARCHAR NOT NULL,
    format VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    source VARCHAR NOT NULL,
    partitions SMALLINT NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);
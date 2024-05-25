-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "data_products" (
  "id" uuid PRIMARY KEY NOT NULL,
  "owner_id" uuid,
  "status" varchar,
  "type" varchar,
  "category" varchar,
  "partitions" smallint,
  "created_at" timestamp,
  "update_at" timestamp
);
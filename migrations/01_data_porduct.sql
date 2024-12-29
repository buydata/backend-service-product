-- Your SQL goes here
DROP TABLE IF EXISTS "data_products";
CREATE TABLE "data_products" (
  "id" uuid PRIMARY KEY NOT NULL,
  "owner_id" uuid  NOT NULL,
  "status" varchar NOT NULL,
  "format" varchar NOT NULL,
  "name" varchar NOT NULL,
  "category" varchar NOT NULL,
  "source" varchar NOT NULL,
  "partitions" int NOT NULL,
  "created_at" timestamp NOT NULL,
  "update_at" timestamp NOT NULL
);
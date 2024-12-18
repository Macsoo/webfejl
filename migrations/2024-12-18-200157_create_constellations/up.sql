-- Your SQL goes here
ALTER TABLE "star" ADD COLUMN "constellation_id" INT4;

CREATE TABLE "constellation"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL
);


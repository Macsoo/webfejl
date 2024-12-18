-- This file should undo anything in `up.sql`
ALTER TABLE "star" DROP COLUMN "constellation_id";

DROP TABLE IF EXISTS "constellation";

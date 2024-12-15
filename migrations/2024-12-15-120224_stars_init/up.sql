-- Your SQL goes here
CREATE TABLE "star"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"right_ascension" FLOAT8 NOT NULL,
	"declination" FLOAT8 NOT NULL,
	"light_years" FLOAT4 NOT NULL,
	"magnitude" FLOAT4 NOT NULL,
	"bv_value" FLOAT4 NOT NULL
);


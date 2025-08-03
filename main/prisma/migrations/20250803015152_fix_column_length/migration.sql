/*
  Warnings:

  - Made the column `taker` on table `orders` required. This step will fail if there are existing NULL values in that column.
  - Made the column `hashlock` on table `orders` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "public"."orders" ALTER COLUMN "salt" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "maker" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "taker" SET NOT NULL,
ALTER COLUMN "taker" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "makingAsset" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "takingAsset" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "r" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "vs" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "hashlock" SET NOT NULL,
ALTER COLUMN "hashlock" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "orderHash" SET DATA TYPE VARCHAR(255);

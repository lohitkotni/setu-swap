/*
  Warnings:

  - You are about to alter the column `salt` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(66)`.
  - You are about to alter the column `maker` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(42)`.
  - You are about to alter the column `taker` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(42)`.
  - You are about to alter the column `makingAsset` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(42)`.
  - You are about to alter the column `takingAsset` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(56)`.
  - You are about to alter the column `r` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(66)`.
  - You are about to alter the column `vs` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(66)`.
  - You are about to alter the column `hashlock` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(66)`.
  - You are about to alter the column `orderHash` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(66)`.

*/
-- AlterTable
ALTER TABLE "public"."orders" ADD COLUMN     "destinationChainId" INTEGER NOT NULL DEFAULT 333,
ADD COLUMN     "makerBalance" TEXT,
ADD COLUMN     "makerTraits" INTEGER NOT NULL DEFAULT 0,
ADD COLUMN     "sourceChainId" INTEGER NOT NULL DEFAULT 1,
ALTER COLUMN "salt" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "maker" SET DATA TYPE VARCHAR(42),
ALTER COLUMN "taker" DROP NOT NULL,
ALTER COLUMN "taker" SET DATA TYPE VARCHAR(42),
ALTER COLUMN "makingAsset" SET DATA TYPE VARCHAR(42),
ALTER COLUMN "takingAsset" SET DATA TYPE VARCHAR(56),
ALTER COLUMN "r" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "vs" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "hashlock" DROP NOT NULL,
ALTER COLUMN "hashlock" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "orderHash" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "makingAmount" SET DATA TYPE TEXT,
ALTER COLUMN "takingAmount" SET DATA TYPE TEXT;

-- CreateIndex
CREATE INDEX "orders_sourceChainId_idx" ON "public"."orders"("sourceChainId");

-- CreateIndex
CREATE INDEX "orders_destinationChainId_idx" ON "public"."orders"("destinationChainId");

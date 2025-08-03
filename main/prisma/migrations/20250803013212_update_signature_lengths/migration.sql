/*
  Warnings:

  - You are about to alter the column `salt` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(66)`.
  - You are about to alter the column `r` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(66)`.
  - You are about to alter the column `vs` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(68)`.
  - You are about to alter the column `hashlock` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(66)`.
  - The `makingAmount` column on the `orders` table would be dropped and recreated. This will lead to data loss if there is data in the column.
  - The `takingAmount` column on the `orders` table would be dropped and recreated. This will lead to data loss if there is data in the column.
  - Changed the type of `filledAmount` on the `order_fills` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- AlterTable
ALTER TABLE "public"."order_fills" DROP COLUMN "filledAmount",
ADD COLUMN     "filledAmount" DECIMAL(78,18) NOT NULL,
ALTER COLUMN "createdAt" SET DATA TYPE TIMESTAMPTZ;

-- AlterTable
ALTER TABLE "public"."orders" ALTER COLUMN "salt" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "r" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "vs" SET DATA TYPE VARCHAR(68),
ALTER COLUMN "hashlock" SET DATA TYPE VARCHAR(66),
DROP COLUMN "makingAmount",
ADD COLUMN     "makingAmount" DECIMAL(78,18),
DROP COLUMN "takingAmount",
ADD COLUMN     "takingAmount" DECIMAL(78,18),
ALTER COLUMN "createdAt" SET DATA TYPE TIMESTAMPTZ,
ALTER COLUMN "updatedAt" SET DATA TYPE TIMESTAMPTZ,
ALTER COLUMN "expiresAt" SET DATA TYPE TIMESTAMPTZ;

-- CreateIndex
CREATE INDEX "order_fills_orderId_idx" ON "public"."order_fills"("orderId");

-- CreateIndex
CREATE INDEX "order_fills_filledBy_idx" ON "public"."order_fills"("filledBy");

-- CreateIndex
CREATE INDEX "order_fills_txHash_idx" ON "public"."order_fills"("txHash");

-- CreateIndex
CREATE INDEX "order_fills_blockNumber_idx" ON "public"."order_fills"("blockNumber");

-- CreateIndex
CREATE INDEX "orders_maker_idx" ON "public"."orders"("maker");

-- CreateIndex
CREATE INDEX "orders_orderHash_idx" ON "public"."orders"("orderHash");

-- CreateIndex
CREATE INDEX "orders_status_idx" ON "public"."orders"("status");

-- CreateIndex
CREATE INDEX "orders_expiresAt_idx" ON "public"."orders"("expiresAt");

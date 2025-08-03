/*
  Warnings:

  - The primary key for the `order_fills` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The `id` column on the `order_fills` table would be dropped and recreated. This will lead to data loss if there is data in the column.
  - The primary key for the `orders` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The `id` column on the `orders` table would be dropped and recreated. This will lead to data loss if there is data in the column.
  - You are about to alter the column `makerBalance` on the `orders` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - Changed the type of `orderId` on the `order_fills` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- DropForeignKey
ALTER TABLE "public"."order_fills" DROP CONSTRAINT "order_fills_orderId_fkey";

-- AlterTable
ALTER TABLE "public"."order_fills" DROP CONSTRAINT "order_fills_pkey",
DROP COLUMN "id",
ADD COLUMN     "id" SERIAL NOT NULL,
DROP COLUMN "orderId",
ADD COLUMN     "orderId" INTEGER NOT NULL,
ADD CONSTRAINT "order_fills_pkey" PRIMARY KEY ("id");

-- AlterTable
ALTER TABLE "public"."orders" DROP CONSTRAINT "orders_pkey",
DROP COLUMN "id",
ADD COLUMN     "id" SERIAL NOT NULL,
ALTER COLUMN "makerBalance" SET DATA TYPE VARCHAR(255),
ADD CONSTRAINT "orders_pkey" PRIMARY KEY ("id");

-- CreateIndex
CREATE INDEX "order_fills_orderId_idx" ON "public"."order_fills"("orderId");

-- AddForeignKey
ALTER TABLE "public"."order_fills" ADD CONSTRAINT "order_fills_orderId_fkey" FOREIGN KEY ("orderId") REFERENCES "public"."orders"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

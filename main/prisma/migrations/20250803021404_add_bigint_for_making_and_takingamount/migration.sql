/*
  Warnings:

  - You are about to drop the column `makingAsset` on the `orders` table. All the data in the column will be lost.
  - You are about to drop the column `takingAsset` on the `orders` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "public"."orders" DROP COLUMN "makingAsset",
DROP COLUMN "takingAsset",
ADD COLUMN     "makingAmount" DECIMAL(78,18),
ADD COLUMN     "takingAmount" DECIMAL(78,18);

/*
  Warnings:

  - You are about to drop the column `makingAmount` on the `orders` table. All the data in the column will be lost.
  - You are about to drop the column `takingAmount` on the `orders` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "public"."orders" DROP COLUMN "makingAmount",
DROP COLUMN "takingAmount";

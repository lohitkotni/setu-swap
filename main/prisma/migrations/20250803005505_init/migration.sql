-- CreateEnum
CREATE TYPE "public"."OrderStatus" AS ENUM ('OPEN', 'FILLED', 'CANCELLED', 'EXPIRED');

-- CreateTable
CREATE TABLE "public"."orders" (
    "id" TEXT NOT NULL,
    "salt" TEXT NOT NULL,
    "maker" VARCHAR(42) NOT NULL,
    "taker" VARCHAR(42),
    "makingAsset" VARCHAR(42) NOT NULL,
    "takingAsset" VARCHAR(42) NOT NULL,
    "r" TEXT NOT NULL,
    "vs" TEXT NOT NULL,
    "hashlock" TEXT,
    "orderHash" VARCHAR(66) NOT NULL,
    "makingAmount" TEXT,
    "takingAmount" TEXT,
    "status" "public"."OrderStatus" NOT NULL DEFAULT 'OPEN',
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,
    "expiresAt" TIMESTAMP(3),

    CONSTRAINT "orders_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "public"."order_fills" (
    "id" TEXT NOT NULL,
    "orderId" TEXT NOT NULL,
    "filledBy" VARCHAR(42) NOT NULL,
    "filledAmount" TEXT NOT NULL,
    "txHash" VARCHAR(66) NOT NULL,
    "blockNumber" INTEGER NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "order_fills_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "orders_orderHash_key" ON "public"."orders"("orderHash");

-- AddForeignKey
ALTER TABLE "public"."order_fills" ADD CONSTRAINT "order_fills_orderId_fkey" FOREIGN KEY ("orderId") REFERENCES "public"."orders"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

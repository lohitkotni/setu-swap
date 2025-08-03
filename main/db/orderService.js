import { PrismaClient } from "@prisma/client";
import { makeError } from "ethers";
const prisma = new PrismaClient();

export async function createOrder(orderData) {
  const expiresAt = orderData.expiresInSeconds
    ? new Date(Date.now() + orderData.expiresInSeconds * 1000)
    : null;
  try {
    const order = await prisma.order.create({
      data: {
        salt: orderData.salt,
        maker: orderData.maker,
        taker: orderData.taker || null, // Optional field
        makingAmount: orderData.makingAmount || null, // For ERC20 tokens (18 decimals)
        takingAmount: orderData.takingAmount || null, // For ERC20 tokens (18 decimals)
        r: orderData.r,
        vs: orderData.vs,
        hashlock: orderData.hashlock || null,
        makerBalance: orderData.makerBalance,
        orderHash: orderData.orderHash,
        sourceChainId: orderData.sourceChainId,
        destinationChainId: orderData.destinationChainId,
        status: orderData.status || "OPEN",
        expiresAt: expiresAt,
        makerTraits: orderData.makerTraits,
      },
    });
    return order;
  } catch (error) {
    console.error("Error creating order:", error);
    throw error;
  } finally {
    await prisma.$disconnect();
  }
}

export async function getActiveOrders(filters = {}) {
  try {
    const { sourceChainId, destinationChainId } = filters;

    // Build the where clause for the query
    const where = {
      status: "OPEN",
      // Only include non-expired orders
      OR: [{ expiresAt: null }, { expiresAt: { gt: new Date() } }],
    };

    // Add optional chain filters if provided
    if (sourceChainId !== undefined) {
      where.sourceChainId = sourceChainId;
    }
    if (destinationChainId !== undefined) {
      where.destinationChainId = destinationChainId;
    }

    const orders = await prisma.order.findMany({
      where,
      orderBy: {
        createdAt: "desc", // Most recent orders first
      },
    });

    return orders;
  } catch (error) {
    console.error("Error fetching active orders:", error);
    throw error;
  } finally {
    await prisma.$disconnect();
  }
}

import { ethers } from "ethers";
import { EscrowData, getDatabase } from "./database";

// Shared interfaces
export interface EscrowEvent {
  type: "EscrowCreated" | "FundsClaimed" | "OrderCancelled" | "FundsRescued";
  escrowAddress: string;
  maker: string;
  taker: string;
  token: string;
  amount: string;
  hashlock: string;
  timelock: number;
  preimage?: string;
  blockNumber: number;
  transactionHash: string;
  chainId: number;
}

export interface ListenerConfig {
  rpcUrl: string;
  factoryAddress: string;
  chainId: number;
  startBlock?: number;
}

export interface SrcImmutables {
  orderHash: string;
  hashlock: string;
  maker: bigint;
  taker: bigint;
  token: bigint;
  amount: bigint;
  safetyDeposit: bigint;
  timelocks: bigint;
}

export interface DstImmutablesComplement {
  maker: bigint;
  amount: bigint;
  token: bigint;
  safetyDeposit: bigint;
  chainId: bigint;
}

export const ESCROW_FACTORY_ABI = [
  "event SrcEscrowCreated(tuple(bytes32 orderHash, bytes32 hashlock, uint256 maker, uint256 taker, uint256 token, uint256 amount, uint256 safetyDeposit, uint256 timelocks) srcImmutables, tuple(uint256 maker, uint256 amount, uint256 token, uint256 safetyDeposit, uint256 chainId) dstImmutablesComplement)",
  "event DstEscrowCreated(address escrow, bytes32 hashlock, uint256 taker)",
  "function addressOfEscrowSrc(tuple(bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256) immutables) external view returns (address)",
  "function addressOfEscrowDst(tuple(bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256) immutables) external view returns (address)",
];

export const BASE_ESCROW_ABI = [
  "event EscrowWithdrawal(bytes32 secret)",
  "event EscrowCancelled()",
  "event FundsRescued(address token, uint256 amount)",
  "function FACTORY() external view returns (address)",
  "function RESCUE_DELAY() external view returns (uint256)",
  "function getImmutables() external view returns (tuple(bytes32,bytes32,uint256,uint256,uint256,uint256,uint256,uint256))",
];

export function createFactoryContract(
  factoryAddress: string,
  provider: ethers.JsonRpcProvider
) {
  return new ethers.Contract(factoryAddress, ESCROW_FACTORY_ABI, provider);
}

export function createEscrowContract(
  escrowAddress: string,
  provider: ethers.JsonRpcProvider
) {
  return new ethers.Contract(escrowAddress, BASE_ESCROW_ABI, provider);
}

export async function calculateEscrowAddress(
  srcImmutables: SrcImmutables,
  factoryContract: ethers.Contract
): Promise<string> {
  const immutablesTuple = [
    srcImmutables.orderHash,
    srcImmutables.hashlock,
    srcImmutables.maker,
    srcImmutables.taker,
    srcImmutables.token,
    srcImmutables.amount,
    srcImmutables.safetyDeposit,
    srcImmutables.timelocks,
  ];
  return await factoryContract.addressOfEscrowSrc(immutablesTuple);
}

export function parseTimelocks(timelocksBigInt: bigint) {
  const deployedAt = Number(timelocksBigInt & BigInt(0xffffffff));
  const srcWithdrawal = Number(
    (timelocksBigInt >> BigInt(32)) & BigInt(0xffffffff)
  );
  const srcCancellation = Number(
    (timelocksBigInt >> BigInt(64)) & BigInt(0xffffffff)
  );
  const dstWithdrawal = Number(
    (timelocksBigInt >> BigInt(96)) & BigInt(0xffffffff)
  );
  const dstCancellation = Number(
    (timelocksBigInt >> BigInt(128)) & BigInt(0xffffffff)
  );
  const now = Math.floor(Date.now() / 1000);
  const timestamp = Number(timelocksBigInt);
  if (timestamp > now - 86400 && timestamp < now + 86400 * 365) {
    return {
      deployedAt: timestamp,
      srcWithdrawal: timestamp + 3600,
      srcCancellation: timestamp + 7200,
      dstWithdrawal: timestamp + 3600,
      dstCancellation: timestamp + 7200,
    };
  }
  return {
    deployedAt,
    srcWithdrawal,
    srcCancellation,
    dstWithdrawal,
    dstCancellation,
  };
}

export async function recordEscrow(
  srcImmutables: SrcImmutables,
  dstImmutablesComplement: DstImmutablesComplement,
  event: ethers.EventLog,
  chainId: number,
  factoryContract: ethers.Contract
): Promise<EscrowEvent> {
  const escrowAddress = await calculateEscrowAddress(
    srcImmutables,
    factoryContract
  );
  const timelocks = parseTimelocks(srcImmutables.timelocks);
  const db = await getDatabase();

  const escrowData: EscrowData = {
    escrowAddress,
    hashlock: srcImmutables.hashlock,
    maker: srcImmutables.maker.toString(),
    taker: srcImmutables.taker.toString(),
    token: srcImmutables.token.toString(),
    amount: srcImmutables.amount.toString(),
    safetyDeposit: srcImmutables.safetyDeposit.toString(),
    timelocks,
    orderHash: srcImmutables.orderHash,
    chainId,
    status: "pending",
    createdAt: Date.now(),
    updatedAt: Date.now(),
  };
  await db.recordEscrow(escrowData);

  return {
    type: "EscrowCreated",
    escrowAddress,
    maker: escrowData.maker,
    taker: escrowData.taker,
    token: escrowData.token,
    amount: escrowData.amount,
    hashlock: escrowData.hashlock,
    timelock: timelocks.srcWithdrawal,
    blockNumber: event.blockNumber,
    transactionHash: event.transactionHash,
    chainId,
  };
}

export async function saveEvent(event: EscrowEvent): Promise<void> {
  const db = await getDatabase();
  await db.recordEvent({
    escrowAddress: event.escrowAddress,
    type: event.type,
    hashlock: event.hashlock,
    preimage: event.preimage,
    blockNumber: event.blockNumber,
    transactionHash: event.transactionHash,
    chainId: event.chainId,
    data: {
      maker: event.maker,
      taker: event.taker,
      token: event.token,
      amount: event.amount,
      timelock: event.timelock,
    },
  });

  if (event.type === "FundsClaimed") {
    await db.updateEscrowStatus(
      event.escrowAddress,
      event.chainId,
      "withdrawn"
    );
  } else if (event.type === "OrderCancelled") {
    await db.updateEscrowStatus(
      event.escrowAddress,
      event.chainId,
      "cancelled"
    );
  }
  console.log(`✅ Event saved: ${event.type} on chain ${event.chainId}`);
}

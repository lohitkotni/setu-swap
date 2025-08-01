import path from "path";
import { promises } from "fs";

export interface EscrowData {
  escrowAddress: string;
  hashlock: string;
  maker: string;
  taker: string;
  token: string;
  amount: string;
  safetyDeposit: string;
  timelocks: {
    deployedAt: number;
    srcWithdrawal: number;
    srcCancellation: number;
    dstWithdrawal: number;
    dstCancellation: number;
  };
  orderHash: string;
  chainId: number;
  status: "pending" | "funded" | "withdrawn" | "cancelled" | "rescued";
  createdAt: number;
  updatedAt: number;
}

export interface EscrowEvent {
  id: string;
  escrowAddress: string;
  type: "EscrowCreated" | "FundsClaimed" | "OrderCancelled" | "FundsRescued";
  hashlock: string;
  preimage?: string;
  blockNumber: number;
  transactionHash: string;
  chainId: number;
  timestamp: number;
  data?: Record<string, unknown>;
}

interface DatabaseFile {
  escrows: EscrowData[];
  events: EscrowEvent[];
  lastProcessedBlock: Record<number, number>;
}

const defaultContent: DatabaseFile = {
  escrows: [],
  events: [],
  lastProcessedBlock: {},
};

class LocalJsonStore {
  private filePath: string;
  private memory: DatabaseFile;

  constructor() {
    this.filePath = path.join(process.cwd(), "data", "store.json");
    this.memory = { ...defaultContent };
  }

  async initialize(): Promise<void> {
    try {
      const raw = await promises.readFile(this.filePath, "utf-8");
      this.memory = JSON.parse(raw);
    } catch (_) {
      this.memory = { ...defaultContent };
      await this.saveToDisk();
    }
  }

  private async saveToDisk(): Promise<void> {
    const json = JSON.stringify(this.memory, null, 2);
    await promises.writeFile(this.filePath, json);
  }

  // --- Escrow Storage ---

  async recordEscrow(escrow: EscrowData): Promise<void> {
    const index = this.memory.escrows.findIndex(
      (e) =>
        e.escrowAddress === escrow.escrowAddress && e.chainId === escrow.chainId
    );

    const now = Date.now();

    if (index >= 0) {
      this.memory.escrows[index] = {
        ...escrow,
        updatedAt: now,
      };
    } else {
      this.memory.escrows.push({
        ...escrow,
        createdAt: now,
        updatedAt: now,
      });
    }

    await this.saveToDisk();
  }

  async getEscrow(
    escrowAddress: string,
    chainId: number
  ): Promise<EscrowData | undefined> {
    return this.memory.escrows.find(
      (e) => e.escrowAddress === escrowAddress && e.chainId === chainId
    );
  }

  async getEscrowByHashlock(
    hashlock: string,
    chainId: number
  ): Promise<EscrowData | undefined> {
    return this.memory.escrows.find(
      (e) => e.hashlock === hashlock && e.chainId === chainId
    );
  }

  async updateEscrowStatus(
    escrowAddress: string,
    chainId: number,
    status: EscrowData["status"]
  ): Promise<void> {
    const escrow = await this.getEscrow(escrowAddress, chainId);
    if (escrow) {
      escrow.status = status;
      escrow.updatedAt = Date.now();
      await this.recordEscrow(escrow);
    }
  }

  async getEscrowsByChain(chainId: number): Promise<EscrowData[]> {
    return this.memory.escrows.filter((e) => e.chainId === chainId);
  }

  async getEscrowsByStatus(
    status: EscrowData["status"]
  ): Promise<EscrowData[]> {
    return this.memory.escrows.filter((e) => e.status === status);
  }

  async getAllEscrows(): Promise<EscrowData[]> {
    return this.memory.escrows;
  }

  // --- Event Storage ---

  async recordEvent(
    event: Omit<EscrowEvent, "id" | "timestamp">
  ): Promise<void> {
    const id = `${event.chainId}-${event.transactionHash}-${event.blockNumber}`;

    if (this.memory.events.some((e) => e.id === id)) {
      console.warn(`⚠️ Event already recorded: ${id}`);
      return;
    }

    this.memory.events.push({
      ...event,
      id,
      timestamp: Date.now(),
    });

    await this.saveToDisk();
  }

  async getEventsForEscrow(
    escrowAddress: string,
    chainId: number
  ): Promise<EscrowEvent[]> {
    return this.memory.events.filter(
      (e) => e.escrowAddress === escrowAddress && e.chainId === chainId
    );
  }

  async getEventsByHashlock(
    hashlock: string,
    chainId: number
  ): Promise<EscrowEvent[]> {
    return this.memory.events.filter(
      (e) => e.hashlock === hashlock && e.chainId === chainId
    );
  }

  async getAllEvents(): Promise<EscrowEvent[]> {
    return this.memory.events;
  }

  // --- Block Processing ---

  async setLastProcessedBlock(
    chainId: number,
    blockNumber: number
  ): Promise<void> {
    this.memory.lastProcessedBlock[chainId] = blockNumber;
    await this.saveToDisk();
  }

  async getLastProcessedBlock(chainId: number): Promise<number> {
    return this.memory.lastProcessedBlock[chainId] ?? 0;
  }
}

// --- Singleton Access ---

let instance: LocalJsonStore | null = null;

export async function getDatabase(): Promise<LocalJsonStore> {
  if (!instance) {
    instance = new LocalJsonStore();
    await instance.initialize();
  }
  return instance;
}

export { LocalJsonStore };

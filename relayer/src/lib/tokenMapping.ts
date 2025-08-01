export interface TokenMapping {
  localAddress: string;
  mainnetAddress: string;
  symbol: string;
  name: string;
  decimals: number;
}

export const TOKEN_MAPPINGS: TokenMapping[] = [
  // MEME Token
  {
    localAddress:
      process.env.MEME_TOKEN || "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0",
    mainnetAddress: "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9", // Real AAVE mainnet
    symbol: "AAVE",
    name: "Aave Token",
    decimals: 18,
  },
  // WETH Token
  {
    localAddress:
      process.env.STABLE_TOKEN || "0xC7b69070352f04d5aAa33eda4916EdE802db0edf",
    mainnetAddress: "0xC7b69070352f04d5aAa33eda4916EdE802db0edf", // R
    symbol: "STB",
    name: "STABLE TOKEN",
    decimals: 18,
  },
  // Stellar Lumens(XLM)
  {
    localAddress: process.env.STELLAR_ADDRESS || "",
    mainnetAddress:
      "000000000000000000000000000000000000000000000000000000000000",
    symbol: "XLM",
    name: "Stellar Lumens",
    decimals: 8,
  },
  // USDC on Stellar
  {
    localAddress:
      process.env.STELLAR_USDC_ADDRESS ||
      "GBF2342JDAISOFJ34OIEHRFGJOI34HFGJ45JRF78FGJOI34OIERHGLKJA45J",
    mainnetAddress:
      "GBF2342JDAISOFJ34OIEHRFGJOI34HFGJ45JRF78FGJOI34OIERHGLKJA45J",
    symbol: "USDC",
    name: "USD Coin",
    decimals: 6,
  },
];

// Helper function to get mainnet address from local address
export function getMainnetAddress(localAddress: string): string | null {
  const mapping = TOKEN_MAPPINGS.find(
    (token) => token.localAddress.toLowerCase() === localAddress.toLowerCase()
  );
  return mapping?.mainnetAddress || null;
}

// Helper function to get token info by local address
export function getTokenInfo(localAddress: string): TokenMapping | null {
  return (
    TOKEN_MAPPINGS.find(
      (token) => token.localAddress.toLowerCase() === localAddress.toLowerCase()
    ) || null
  );
}

// Helper function to get token info by mainnet address
export function getTokenInfoByMainnet(
  mainnetAddress: string
): TokenMapping | null {
  return (
    TOKEN_MAPPINGS.find(
      (token) =>
        token.mainnetAddress.toLowerCase() === mainnetAddress.toLowerCase()
    ) || null
  );
}

// Get all supported tokens
export function getAllTokens(): TokenMapping[] {
  return TOKEN_MAPPINGS;
}

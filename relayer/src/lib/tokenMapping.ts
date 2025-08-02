export interface TokenMapping {
  localAddress: string;
  mainnetAddress: string;
  symbol: string;
  name: string;
  decimals: number;
}

export const TOKEN_MAPPINGS: TokenMapping[] = [
  {
    localAddress:
      process.env.NEXT_PUBLIC_1INCH_ADDRESS ||
      " 0x5FbDB2315678afecb367f032d93F642f64180aa3",
    mainnetAddress: "0x111111111117dC0aa78b770fA6A738034120C302",
    symbol: "1INCH",
    name: "1INCH",
    decimals: 18,
  },
  {
    localAddress:
      process.env.NEXT_PUBLIC_USDC_ADDRESS ||
      "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0",
    mainnetAddress: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", //
    symbol: "USDC",
    name: "USDC",
    decimals: 18,
  },

  {
    localAddress:
      process.env.NEXT_PUBLIC_AAVE_ADDRESS ||
      "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
    mainnetAddress: "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9",
    symbol: "AAVE",
    name: "Aave Token",
    decimals: 18,
  },
  // Stellar Lumens(XLM)
  {
    localAddress: process.env.NEXT_PUBLIC_STELLAR_ADDRESS || "",
    mainnetAddress:
      "000000000000000000000000000000000000000000000000000000000000",
    symbol: "XLM",
    name: "Stellar Lumens",
    decimals: 8,
  },
  // USDC on Stellar
  {
    localAddress:
      process.env.NEXT_PUBLIC_STELLAR_USDC_ADDRESS ||
      "CAI267TPLO3BX2TX5OYAGT6OYCHDKY3JHLYLPASI6FIFSVFIJTZKN2NC",
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

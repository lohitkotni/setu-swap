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
      process.env.NEXT_PUBLIC_1INCH_ADDRESS ||
      "0x5fbdb2315678afecb367f032d93f642f64180aa3",
    mainnetAddress: "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9",
    symbol: "1INCH",
    name: "1INCH",
    decimals: 18,
  },
  // WETH Token
  {
    localAddress:
      process.env.NEXT_PUBLIC_USDC_ADDRESS ||
      "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512",
    mainnetAddress: "0x59EaF0e1E480fE7D1d07794EdB637930CEc77591", //
    symbol: "USDC",
    name: "USDC",
    decimals: 18,
  },

  {
    localAddress:
      process.env.NEXT_PUBLIC_AAVE_ADDRESS ||
      "0x2de2002Cf9c3d9f92Cccc6CB71aCDb6d7Cd7BbaA",
    mainnetAddress: "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9",
    symbol: "AAVE",
    name: "Aave Token",
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

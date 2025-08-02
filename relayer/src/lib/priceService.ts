// 1inch API price fetching service
// Based on documentation: https://portal.1inch.dev/documentation/apis/spot-price/quick-start

import { getMainnetAddress, getTokenInfo } from "./tokenMapping";

// 1inch API base URL for Ethereum mainnet
const ONEINCH_API_BASE = "https://1inch-vercel-proxy-ruby.vercel.app/";
const ETHEREUM_CHAIN_ID = 1;

// Types for 1inch API responses
export interface TokenPrice {
  [tokenAddress: string]: string; // Price in USD
}

export interface PriceResponse {
  [tokenAddress: string]: string;
}

export interface QuoteParams {
  srcToken: string;
  dstToken: string;
  amount: string;
}

export interface QuoteResponse {
  dstAmount: string;
  srcToken: {
    address: string;
    symbol: string;
    name: string;
    decimals: number;
  };
  dstToken: {
    address: string;
    symbol: string;
    name: string;
    decimals: number;
  };
  protocols: Array<any>;
}

// Mock prices for fallback when API is unavailable
const MOCK_PRICES: Record<string, string> = {
  // 1INCH Token
  "0x0A089d17D94eD55283d3c9087C22F30430078B75": "0.32",
  // USDC
  "0x59EaF0e1E480fE7D1d07794EdB637930CEc77591": "1.00",
  // AAVE
  "0x2de2002Cf9c3d9f92Cccc6CB71aCDb6d7Cd7BbaA": "89.50",
  // WETH
  "0x14Dd0dDcC0f8E21DD60a595AC30A07a0132B0e3b": "3584.00",
  // Stellar
  "00000000000000000000000000000000000000000000000000000000": "0.14",
  // XLM_USDC
  CAI267TPLO3BX2TX5OYAGT6OYCHDKY3JHLYLPASI6FIFSVFIJTZKN2NC: "1",
};

/**
 * Get mock prices for tokens (fallback when API is unavailable)
 */
function getMockPrices(tokenAddresses: string[]): TokenPrice {
  const result: TokenPrice = {};
  tokenAddresses.forEach((address) => {
    result[address] = MOCK_PRICES[address] || "1.00";
  });
  return result;
}

// Get API headers with authorization
function getApiHeaders(): HeadersInit {
  const apiKey = process.env.ONEINCH_API_KEY;

  const headers: HeadersInit = {
    "Content-Type": "application/json",
  };

  if (apiKey) {
    headers["Authorization"] = `Bearer ${apiKey}`;
  }

  return headers;
}

/**
 * Fetch current USD prices for multiple tokens
 * @param tokenAddresses Array of token addresses (can be local or mainnet)
 * @returns Promise<TokenPrice> - Object mapping addresses to USD prices
 */
export async function getTokenPrices(
  tokenAddresses: string[]
): Promise<TokenPrice> {
  try {
    // Convert local addresses to mainnet addresses
    const mainnetAddresses = tokenAddresses
      .map((addr) => getMainnetAddress(addr) || addr)
      .filter(Boolean);

    if (mainnetAddresses.length === 0) {
      throw new Error("No valid token addresses provided");
    }

    // Build query parameters
    const params = new URLSearchParams({
      tokens: mainnetAddresses.join(","),
      currency: "USD",
    });

    const response = await fetch(
      `${ONEINCH_API_BASE}/${ETHEREUM_CHAIN_ID}?${params}`,
      {
        method: "GET",
        headers: getApiHeaders(),
      }
    );

    if (!response.ok) {
      // If API fails, log the error and return mock prices
      console.warn(
        `1inch API error: ${response.status} ${response.statusText}, using mock prices`
      );

      // Map mock prices back to original addresses
      const mockResult: TokenPrice = {};
      tokenAddresses.forEach((originalAddr) => {
        const mainnetAddr = getMainnetAddress(originalAddr) || originalAddr;
        mockResult[originalAddr] = MOCK_PRICES[mainnetAddr] || "1.00";
      });

      return mockResult;
    }

    const data: PriceResponse = await response.json();

    // Map back to original addresses if they were local
    const result: TokenPrice = {};
    tokenAddresses.forEach((originalAddr) => {
      const mainnetAddr = getMainnetAddress(originalAddr) || originalAddr;
      if (data[mainnetAddr]) {
        result[originalAddr] = data[mainnetAddr];
      } else {
        // Use mock price if not found in API response
        result[originalAddr] = MOCK_PRICES[mainnetAddr] || "1.00";
      }
    });

    return result;
  } catch (error) {
    console.warn("Error fetching token prices, using mock prices:", error);

    // Return mock prices for all requested tokens
    const mockResult: TokenPrice = {};
    tokenAddresses.forEach((originalAddr) => {
      const mainnetAddr = getMainnetAddress(originalAddr) || originalAddr;
      mockResult[originalAddr] = MOCK_PRICES[mainnetAddr] || "1.00";
    });

    return mockResult;
  }
}

/**
 * Get price for a single token in USD
 * @param tokenAddress Token address (local or mainnet)
 * @returns Promise<string> - Price in USD
 */
export async function getTokenPrice(tokenAddress: string): Promise<string> {
  const prices = await getTokenPrices([tokenAddress]);
  const price = prices[tokenAddress];

  if (!price) {
    throw new Error(`Price not found for token ${tokenAddress}`);
  }

  return price;
}

/**
 * Get a quote for swapping tokens (amount of dstToken for given srcToken amount)
 * @param params Quote parameters
 * @returns Promise<QuoteResponse>
 */
export async function getSwapQuote(
  params: QuoteParams
): Promise<QuoteResponse> {
  try {
    // Convert local addresses to mainnet
    const srcMainnet = getMainnetAddress(params.srcToken) || params.srcToken;
    const dstMainnet = getMainnetAddress(params.dstToken) || params.dstToken;

    const queryParams = new URLSearchParams({
      src: srcMainnet,
      dst: dstMainnet,
      amount: params.amount,
    });

    const response = await fetch(
      `${ONEINCH_API_BASE}/${ETHEREUM_CHAIN_ID}/quote?${queryParams}`,
      {
        method: "GET",
        headers: getApiHeaders(),
      }
    );

    if (!response.ok) {
      throw new Error(
        `1inch quote API error: ${response.status} ${response.statusText}`
      );
    }

    const data: QuoteResponse = await response.json();
    return data;
  } catch (error) {
    console.error("Error fetching swap quote:", error);
    throw error;
  }
}

/**
 * Calculate USD value of a token amount
 * @param tokenAddress Token address (local or mainnet)
 * @param amount Amount in token's smallest unit (wei for 18 decimals)
 * @returns Promise<number> - USD value
 */
export async function calculateUSDValue(
  tokenAddress: string,
  amount: string
): Promise<number> {
  try {
    const price = await getTokenPrice(tokenAddress);
    const tokenInfo = getTokenInfo(tokenAddress);

    if (!tokenInfo) {
      throw new Error(`Token info not found for ${tokenAddress}`);
    }

    // Convert amount from smallest unit to readable amount
    const readableAmount =
      parseFloat(amount) / Math.pow(10, tokenInfo.decimals);

    // Calculate USD value
    const usdValue = readableAmount * parseFloat(price);

    return usdValue;
  } catch (error) {
    console.error("Error calculating USD value:", error);
    throw error;
  }
}

/**
 * Get price ratio between two tokens
 * @param token1Address First token address
 * @param token2Address Second token address
 * @returns Promise<number> - Ratio of token1/token2
 */
export async function getPriceRatio(
  token1Address: string,
  token2Address: string
): Promise<number> {
  try {
    const prices = await getTokenPrices([token1Address, token2Address]);

    const price1 = parseFloat(prices[token1Address]);
    const price2 = parseFloat(prices[token2Address]);

    if (!price1 || !price2) {
      throw new Error("Could not fetch prices for both tokens");
    }

    return price1 / price2;
  } catch (error) {
    console.error("Error calculating price ratio:", error);
    throw error;
  }
}

/**
 * Format price for display
 * @param price Price string from API
 * @param decimals Number of decimal places to show
 * @returns string - Formatted price
 */
export function formatPrice(price: string, decimals: number = 6): string {
  const num = parseFloat(price);
  return num.toFixed(decimals);
}

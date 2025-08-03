import axios from "axios";

const BASE_URL = "https://1inch-vercel-proxy-ruby.vercel.app/price/v1.1/1";
const ethPriceInUSD = 3500;

// Generic helper to fetch token price from 1inch proxy
async function fetchTokenPrice(tokenAddress) {
  try {
    const normalized = tokenAddress.toLowerCase();
    const url = `${BASE_URL}/${normalized}`;
    const response = await axios.get(url);
    return response.data[normalized];
  } catch (error) {
    console.error(
      `❌ Failed to fetch price for token ${tokenAddress}:`,
      error.message
    );
    return null;
  }
}

// Fetch token price in USD (via ETH)
export async function getPrice(inputToken) {
  const tokenPriceInETH = await fetchTokenPrice(inputToken);
  return (tokenPriceInETH * ethPriceInUSD) / 10 ** 18;
}

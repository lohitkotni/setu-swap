interface WhitelistConfig {
  tokens: {
    ethereum: string[];
    stellar: string[];
  };
  resolvers: string[];
}

export function getWhitelistConfig(): WhitelistConfig {
  return {
    tokens: {
      ethereum: [
        process.env.NEXT_PUBLIC_1INCH_ADDRESS || "",
        process.env.NEXT_PUBLIC_USDC_ADDRESS || "",
        process.env.NEXT_PUBLIC_AAVE_ADDRESS || "",
        process.env.NEXT_PUBLIC_WETH_ADDRESS || "",
      ].filter(Boolean),
      stellar: [
        process.env.NEXT_PUBLIC_STELLAR_ADDRESS || "",
        process.env.NEXT_PUBLIC_STELLAR_USDC_ADDRESS || "",
      ],
    },
    resolvers: [
      process.env.RESOLVER_ADDRESS || "",
      process.env.RESOLVER_ADDRESS_2 || "",
    ].filter(Boolean),
  };
}

export default getWhitelistConfig();

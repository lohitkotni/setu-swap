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
        process.env.STABLE_TOKEN || "",
        process.env.MEME_TOKEN || "",
        process.env.ACCESS_TOKEN || "",
      ].filter(Boolean),
      stellar: [
        process.env.STELLAR_ADDRESS || "",
        process.env.STELLAR_USDC_ADDRESS || "",
      ],
    },
    resolvers: [
      process.env.RESOLVER_ADDRESS || "",
      process.env.RESOLVER_ADDRESS_2 || "",
    ].filter(Boolean),
  };
}

export default getWhitelistConfig();

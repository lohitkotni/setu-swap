import * as Client from "./packages/escrow_src";

const contract = new Client.Client({
  ...Client.networks.testnet,
  rpcUrl: "https://soroban-testnet.stellar.org:443",
});

const result = await contract.get_access_token();
console.log(result);

import prompts from "prompts";
import axios from "axios";
import chalk from "chalk";
import ora from "ora";
import dotenv from "dotenv";
import { ethers } from "ethers";
import { getPrice } from "./oneinch_utils.js";
import { getStellarTokenBalances } from "./stellarTokenBalances.js";
import { PrismaClient } from "@prisma/client";
import { createOrder } from "../db/orderService.js";
const prisma = new PrismaClient();

dotenv.config();

const contractABI = [
  {
    type: "function",
    name: "allowance",
    inputs: [
      {
        name: "owner",
        type: "address",
        internalType: "address",
      },
      {
        name: "spender",
        type: "address",
        internalType: "address",
      },
    ],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256",
      },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "approve",
    inputs: [
      {
        name: "spender",
        type: "address",
        internalType: "address",
      },
      {
        name: "value",
        type: "uint256",
        internalType: "uint256",
      },
    ],
    outputs: [
      {
        name: "",
        type: "bool",
        internalType: "bool",
      },
    ],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "balanceOf",
    inputs: [
      {
        name: "account",
        type: "address",
        internalType: "address",
      },
    ],
    outputs: [
      {
        name: "",
        type: "uint256",
        internalType: "uint256",
      },
    ],
    stateMutability: "view",
  },
];
const contractAddress = "0x0A089d17D94eD55283d3c9087C22F30430078B75";

export async function ethereumToStellar() {
  const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
  const signer = new ethers.Wallet(process.env.PRIVATE_KEY, provider);
  const maker = signer.address;
  console.log(chalk.green(`${signer.address} is connected`));

  const makerAsset = await prompts({
    type: "select",
    name: "inputToken",
    message: "Enter the input token from Ethereum",
    choices: [
      { title: "1INCH", value: "0x111111111117dC0aa78b770fA6A738034120C302" },
      { title: "AAVE", value: "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9" },
      { title: "USDC", value: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48" },
    ],
  });

  const contract = new ethers.Contract(contractAddress, contractABI, provider);

  if (makerAsset.inputToken === "0x111111111117dC0aa78b770fA6A738034120C302") {
    // console.log(
    //   `The current price of 1inch is ${await getPrice(
    //     selectInputToken.inputToken
    //   )}`
    // );
  } else if (
    makerAsset.inputToken === "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9"
  ) {
    // console.log(
    //   `The current price of Aave is ${await getPrice(
    //     selectInputToken.inputToken
    //   )}`
    // );
  } else if (
    makerAsset.inputToken === "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
  ) {
    // console.log(
    //   `The current price of USDC is ${await getPrice(
    //     selectInputToken.inputToken
    //   )}`
    // );
  }
  const makerBalance = await contract.balanceOf(maker);
  console.log("Your current balance of the input token is " + makerBalance);

  const takerAsset = await prompts({
    type: "select",
    name: "outputToken",
    message: "Enter the input token from Ethereum",
    choices: [
      {
        title: "XLM",
        value: "GDU4D7BPCGXXELMXN32IFVXDPW5F5V3RBUVZQCCK3A5Y5QXMN3OL5ODR",
      },
      {
        title: "USDC",
        value: "CAI267TPLO3BX2TX5OYAGT6OYCHDKY3JHLYLPASI6FIFSVFIJTZKN2NC",
      },
    ],
  });

  if (
    takerAsset.outputToken ===
    "GDU4D7BPCGXXELMXN32IFVXDPW5F5V3RBUVZQCCK3A5Y5QXMN3OL5ODR"
  ) {
    console.log(
      `The current Price of XLM is ${getStellarTokenBalances("XLM")}`
    );
  } else if (
    takerAsset.outputToken ===
    "CAI267TPLO3BX2TX5OYAGT6OYCHDKY3JHLYLPASI6FIFSVFIJTZKN2NC"
  ) {
    console.log(
      `The current Price of USDC is ${getStellarTokenBalances("USDC")}`
    );
  }

  let makingAmount = await prompts({
    type: "text",
    name: "makingAmount",
    message: "Enter the amount of input token you want to swap",
  });
  makingAmount.makingAmount = BigInt(makingAmount.makingAmount);
  let takingAmount = await prompts({
    type: "text",
    name: "takingAmount",
    message: "Enter the amount of output token you want to get",
  });
  takingAmount.takingAmount = BigInt(takingAmount.takingAmount);

  const slippage = await prompts({
    type: "number",
    name: "slippage",
    message: "Enter the slippage percentage",
  });
  const currentTime = Math.floor(Date.now() / 1000);

  const expiry = await prompts({
    type: "number",
    name: "expiresAt",
    message: "Enter the expiration time in seconds",
  });
  const expiresAt = new Date(Date.now() + expiry.expiresAt * 1000);

  const salt = ethers.keccak256(
    ethers.toUtf8Bytes(
      `${makerAsset.inputToken}-${takerAsset.outputToken}-${makingAmount.makingAmount}-${takingAmount.takingAmount}-${currentTime}-${expiry.expiresAt}`
    )
  );

  const order = ethers.keccak256(
    ethers.toUtf8Bytes(`${salt}-${maker}-${currentTime}`)
  );
  console.log(`Order: ${order}`);

  const secret = ethers.randomBytes(32);
  console.log(`Secret: ${ethers.hexlify(secret)}`);
  console.log(chalk.red("Do not share the secret with anyone"));
  const hashlock = ethers.keccak256(secret);

  const signature = await signer.signMessage(ethers.getBytes(order));
  const sig = ethers.Signature.from(signature);
  const r = sig.r; // Already correct (66 chars)
  const vs = ethers.concat([sig.s, ethers.toBeArray(sig.v)]); // Combine s and v properly
  const makerTraits = 0;
  const sourceChainId = 1;
  const destinationChainId = 4121;

  console.log(`r: ${r}`);
  console.log(`vs: ${vs}`);
  console.log(`hashlock: ${hashlock}`);

  const orderData = {
    salt: salt,
    maker: maker,
    makingAsset: makerAsset.inputToken,
    takingAsset: takerAsset.outputToken,
    r: r,
    vs: vs,
    hashlock: hashlock,
    orderHash: order,
    makingAmount: makingAmount.makingAmount.toString(),
    takingAmount: takingAmount.takingAmount.toString(),
    expiresInSeconds: expiry.expiresAt,
    status: "OPEN",
    expiresAt: expiresAt,
    makerBalance: makerBalance.toString(),
    makerTraits: makerTraits,
    sourceChainId: sourceChainId,
    destinationChainId: destinationChainId,
  };

  try {
    const createdOrder = await createOrder(orderData);
    console.log(chalk.green("Order created successfully"));
    console.log(createdOrder);
  } catch (error) {
    console.error(chalk.red("Error creating order:", error));
  }
}

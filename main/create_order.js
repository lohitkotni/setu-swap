import prompts from "prompts";
import chalk from "chalk";
import ora from "ora";
import { ethereumToStellar } from "./ethereum_to_stellar.js";

async function main() {
  console.log(chalk.cyan("Welcome to the Setu Swap CLI"));

  console.log(
    chalk.yellow(
      "Setu-Swap CLI is a 1inch based fusion+ application to swap tokens across stellar and ethereum"
    )
  );

  const response = await prompts({
    type: "select",
    name: "Swap type",
    message: "Select the type of swap you want to perform",
    choices: [
      { title: "Stellar to Ethereum", value: "stellar_to_ethereum" },
      { title: "Ethereum to Stellar", value: "ethereum_to_stellar" },
    ],
  });
  if (response["Swap type"] === "stellar_to_ethereum") {
    stellarToEthereum();
  } else if (response["Swap type"] === "ethereum_to_stellar") {
    ethereumToStellar();
  }
}
main();

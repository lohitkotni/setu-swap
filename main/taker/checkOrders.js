import { getActiveOrders } from "../db/orderService.js";
import chalk from "chalk";
import prompts from "prompts";

async function checkOrders() {
  console.log(chalk.cyan("Welcome to the Setu Swap CLI"));
  console.log(
    chalk.yellow(
      "Setu-Swap CLI is a 1inch based fusion+ application to swap tokens across stellar and ethereum"
    )
  );

  try {
    const orders = await getActiveOrders();

    if (orders.length === 0) {
      console.log(chalk.yellow("\nNo active orders found"));
      return;
    }

    console.log(chalk.green(`\nFound ${orders.length} active orders:`));

    // Create choices array for the prompt
    const choices = orders.map((order) => ({
      title: `Order ${order.id} | Making: ${order.makingAmount} | Taking: ${order.takingAmount}`,
      description: `Maker: ${order.maker.slice(0, 10)}... | Expires: ${
        order.expiresAt ? new Date(order.expiresAt).toLocaleString() : "Never"
      }`,
      value: order,
    }));

    // Add a cancel option
    choices.push({
      title: "Cancel",
      value: null,
    });

    const response = await prompts({
      type: "select",
      name: "selectedOrder",
      message: "Select an order to fill",
      choices: choices,
      hint: "Use arrow keys to navigate, Enter to select",
    });

    if (!response.selectedOrder) {
      console.log(chalk.yellow("\nOrder selection cancelled"));
      return;
    }

    const selectedOrder = response.selectedOrder;
    console.log(chalk.green("\nSelected Order Details:"));
    console.log(chalk.cyan("Order Hash:"), selectedOrder.orderHash);
    console.log(chalk.cyan("Maker:"), selectedOrder.maker);
    console.log(chalk.cyan("Making Amount:"), selectedOrder.makingAmount);
    console.log(chalk.cyan("Taking Amount:"), selectedOrder.takingAmount);
    console.log(chalk.cyan("Source Chain:"), selectedOrder.sourceChainId);
    console.log(
      chalk.cyan("Destination Chain:"),
      selectedOrder.destinationChainId
    );
    console.log(
      chalk.cyan("Expires At:"),
      selectedOrder.expiresAt
        ? new Date(selectedOrder.expiresAt).toLocaleString()
        : "Never"
    );

    // Confirm the selection
    const confirmation = await prompts({
      type: "confirm",
      name: "value",
      message: "Do you want to fill this order?",
      initial: false,
    });

    if (confirmation.value) {
      // TODO: Call fillOrder function here
      console.log(chalk.green("\nProceeding to fill the order..."));
      // You can import and call your fillOrder function here
    } else {
      console.log(chalk.yellow("\nOrder filling cancelled"));
    }

    if (selectedOrder.sourceChainId === 1) {
      console.log(chalk.green("\nOrder is on Ethereum"));
      createEthEscrow(selectedOrder);
    } else if (selectedOrder.sourceChainId === 4121) {
      console.log(chalk.green("\nOrder is on Stellar"));
      createStellarEscrow(selectedOrder);
    } else {
      console.log(chalk.red("\nOrder is on an unknown chain"));
    }
  } catch (error) {
    console.error(chalk.red("Error fetching or processing orders:", error));
  }
}

checkOrders();

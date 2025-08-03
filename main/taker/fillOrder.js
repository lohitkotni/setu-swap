import { PrismaClient } from "@prisma/client";
import { getActiveOrders } from "../db/orderService.js";
import chalk from "chalk";
import prompts from "prompts";

const prisma = new PrismaClient();

async function main() {
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

    // First, select the type of orders to view
    const typeResponse = await prompts({
      type: "select",
      name: "orderType",
      message: "Select the type of orders you want to view",
      choices: [
        { title: "Stellar to Ethereum", value: "stellar_to_ethereum" },
        { title: "Ethereum to Stellar", value: "ethereum_to_stellar" },
      ],
    });

    // Filter orders based on selected type
    const filteredOrders = orders.filter((order) => {
      if (typeResponse.orderType === "ethereum_to_stellar") {
        return order.sourceChainId === 1 && order.destinationChainId === 4121;
      } else {
        return order.sourceChainId === 4121 && order.destinationChainId === 1;
      }
    });

    if (filteredOrders.length === 0) {
      console.log(
        chalk.yellow(`\nNo active orders found for ${typeResponse.orderType}`)
      );
      return;
    }

    // Create choices array for the prompt
    const choices = filteredOrders.map((order) => ({
      title: `Order ${order.orderHash.slice(0, 10)}... | Making: ${
        order.makingAmount
      } | Taking: ${order.takingAmount}`,
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

    // Select specific order
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
      await fillOrder(selectedOrder);
    } else {
      console.log(chalk.yellow("\nOrder filling cancelled"));
    }
  } catch (error) {
    console.error(chalk.red("Error processing orders:", error));
  } finally {
    await prisma.$disconnect();
  }
}

async function fillOrder(order) {
  try {
    // TODO: Implement the actual order filling logic
    console.log(chalk.green("Starting to fill order:", order.orderHash));

    // Here you would:
    // 1. Verify the order is still valid
    // 2. Check if the user has sufficient balance
    // 3. Execute the swap
    // 4. Update the order status
    // 5. Create a fill record

    throw new Error("Order filling not implemented yet");
  } catch (error) {
    console.error(chalk.red("Error filling order:", error));
    throw error;
  }
}

// Run the main function
main();

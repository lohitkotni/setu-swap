import { getActiveOrders } from "./orderService.js";
import chalk from "chalk";

async function listActiveOrders() {
  try {
    const orders = await getActiveOrders();

    if (orders.length === 0) {
      console.log(chalk.yellow("No active orders found"));
      return;
    }

    console.log(chalk.green(`Found ${orders.length} active orders:\n`));

    orders.forEach((order, index) => {
      console.log(chalk.blue(`Order #${index + 1}:`));
      console.log(`Order Hash: ${order.orderHash}`);
      console.log(`Maker: ${order.maker}`);
      console.log(`Making Amount: ${order.makingAmount}`);
      console.log(`Taking Amount: ${order.takingAmount}`);
      console.log(`Source Chain: ${order.sourceChainId}`);
      console.log(`Destination Chain: ${order.destinationChainId}`);
      console.log(
        `Expires At: ${
          order.expiresAt ? order.expiresAt.toLocaleString() : "Never"
        }`
      );
      console.log("----------------------------------------\n");
    });
  } catch (error) {
    console.error(chalk.red("Error listing orders:", error));
  }
}

// Run the function
listActiveOrders();

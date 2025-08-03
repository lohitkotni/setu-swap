# Setu Swap

A 1inch Fusion+ based cross-chain swap protocol for Ethereum and Stellar networks. Setu Swap enables seamless token swaps between Ethereum and Stellar blockchains using atomic swaps.

## Overview

Setu Swap allows users to:

- Create swap orders from Ethereum to Stellar
- Create swap orders from Stellar to Ethereum
- View active orders
- Fill existing orders

## Getting Started

First, determine your role in the swap:

### Are you a Maker or Taker?

- **Maker**: Creates new swap orders
- **Taker**: Fills existing swap orders

## Environment Setup

1. Create a `.env` file in the root directory
2. Add the following environment variables:

```env
# Required for both Maker and Taker
SEPOLIA_RPC_URL="your_ethereum_sepolia_rpc_url"
PRIVATE_KEY="your_ethereum_wallet_private_key"

npm install

# Database URL (if running locally)
DATABASE_URL="postgresql://username:password@localhost:5432/setuswap"
```

⚠️ **Security Note**: Never commit your `.env` file or share your private keys!

## For Makers

If you want to create new swap orders:

1. Navigate to the maker directory:

   ```bash
   cd main/maker
   ```

2. Run the order creation script:

   ```bash
   node create_order.js
   ```

3. Follow the interactive prompts to:
   - Select swap direction (Ethereum to Stellar or vice versa)
   - Choose input token
   - Specify amounts
   - Set slippage and expiration time

## For Takers

If you want to fill existing orders:

1. Navigate to the taker directory:

   ```bash
   cd main/taker
   ```

2. To view active orders:

   ```bash
   node checkOrders.js
   ```

3. To fill an order:
   ```bash
   node fillOrder.js
   ```
   - Select the type of orders you want to view
   - Choose an order from the list
   - Confirm the details
   - Proceed with filling the order

## Order Flow

1. **Maker** creates an order specifying:

   - Input token (from source chain)
   - Output token (on destination chain)
   - Amount to swap
   - Expected return amount
   - Order expiration time

2. **Taker** fills an order by:
   - Viewing active orders
   - Selecting an order to fill
   - Confirming the swap details
   - Executing the atomic swap

## Supported Networks

- **Ethereum**: Sepolia Testnet
- **Stellar**: Testnet

## Supported Tokens

### Ethereum (Sepolia)

- 1INCH
- AAVE
- USDC

### Stellar (Testnet)

- XLM
- USDC

## Development

The project structure is organized as follows:

```
main/
├── core/           # Core swap logic
├── maker/          # Maker-specific functionality
├── taker/          # Taker-specific functionality
└── db/             # Database operations
```

## Important Notes

- Always verify order details before creating or filling orders
- Ensure you have sufficient balance in your wallet
- Double-check network connections (Sepolia/Stellar Testnet)
- Keep your private keys secure

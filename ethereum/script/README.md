# 🛠️ Token Utility Script (Foundry / Forge)

This script provides ERC20 token utilities using the Foundry framework. You can check token balances, allowances, and approve spenders directly from the command line using environment variables.

---

## 🚀 Usage

### 1. Check Token Balance

Check the balance of an address for a given ERC20 token:

```bash
ACTION=balance \
TOKEN_ADDRESS=0x... \
ACCOUNT_ADDRESS=0x... \
forge script script/TokenManager.s.sol:TokenManager --rpc-url <RPC_URL>
```

✅ **Example**:

```bash
ACTION=balance \
TOKEN_ADDRESS=0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 \
ACCOUNT_ADDRESS=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
forge script script/TokenManager.s.sol:TokenManager --rpc-url http://localhost:8545
```

---

### 2. Check Token Allowance

Check the amount of tokens a spender is allowed to transfer from an owner's account:

```bash
ACTION=allowance \
TOKEN_ADDRESS=0x... \
OWNER_ADDRESS=0x... \
SPENDER_ADDRESS=0x... \
forge script script/TokenManager.s.sol:TokenManager --rpc-url <RPC_URL>
```

✅ **Example**:

```bash
ACTION=allowance \
TOKEN_ADDRESS=0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 \
OWNER_ADDRESS=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
SPENDER_ADDRESS=0x5fc8d32690cc91d4c39d9d3abcbd16989f875707 \
forge script script/TokenManager.s.sol:TokenManager --rpc-url http://localhost:8545
```

---

### 3. Approve Token Allowance

Approve a spender to transfer tokens on behalf of the signer:

```bash
ACTION=approve \
TOKEN_ADDRESS=0x... \
SPENDER_ADDRESS=0x... \
AMOUNT=100 \
forge script script/TokenManager.s.sol:TokenManager --rpc-url <RPC_URL> --broadcast --private-key <YOUR_PRIVATE_KEY>
```

✅ **Example**:

```bash
ACTION=approve \
TOKEN_ADDRESS=0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 \
SPENDER_ADDRESS=0x5fc8d32690cc91d4c39d9d3abcbd16989f875707 \
AMOUNT=100 \
forge script script/TokenManager.s.sol:TokenManager --rpc-url http://localhost:8545 --broadcast --private-key <YOUR_PRIVATE_KEY>
```

> 🔐 You must provide a private key if you're broadcasting a transaction (`approve`).

---

## ⚙️ Environment Variables Summary

| Variable          | Required For       | Description                                 |
| ----------------- | ------------------ | ------------------------------------------- |
| `ACTION`          | all                | One of `balance`, `allowance`, or `approve` |
| `TOKEN_ADDRESS`   | all                | ERC20 token contract address                |
| `ACCOUNT_ADDRESS` | balance            | Account to check balance for                |
| `OWNER_ADDRESS`   | allowance          | Owner address for allowance check           |
| `SPENDER_ADDRESS` | allowance, approve | Spender address                             |
| `AMOUNT`          | approve            | Amount to approve (in token units)          |

---

In a separate terminal, run:

```bash
ACTION=balance \
TOKEN_ADDRESS=... \
ACCOUNT_ADDRESS=... \
forge script script/TokenManager.s.sol:TokenManager --rpc-url http://localhost:8545
```

---

## 📁 Script Location

Make sure your file is at:

```
script/TokenManager.s.sol
```

---

## 🔒 Notes

- `approve` broadcasts a transaction and requires a private key.
- Amounts are human-readable (e.g., `100`, not `100000000000000000000`).
- The script uses a generic `IERC20` interface and auto-detects decimals/symbol safely.
- All operations are console-logged for clarity.

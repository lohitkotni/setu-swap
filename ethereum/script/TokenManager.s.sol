// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";

interface IERC20 {
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
    function balanceOf(address account) external view returns (uint256);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
}

// You would need to import your actual contract interfaces
// import "./interfaces/USDCoin.sol";
// import "./interfaces/OneInchToken.sol";
// import "./interfaces/AaveToken.sol";
// import "./interfaces/WrappedEther.sol";
// import "./interfaces/UniswapToken.sol";

contract TokenManager is Script {
    function run() external {
        string memory action = vm.envString("ACTION");
        
        if (keccak256(bytes(action)) == keccak256(bytes("balance"))) {
            checkBalance();
        } else if (keccak256(bytes(action)) == keccak256(bytes("allowance"))) {
            checkAllowance();
        } else if (keccak256(bytes(action)) == keccak256(bytes("approve"))) {
            approveToken();
        } else {
            console.log("Error: Invalid action. Use 'balance', 'allowance', or 'approve'");
        }
    }
    
    function checkBalance() internal view {
        address token = vm.envAddress("TOKEN_ADDRESS");
        address account = vm.envAddress("ACCOUNT_ADDRESS");
        
        IERC20 tokenContract = getTokenContract(token);
        
        try tokenContract.balanceOf(account) returns (uint256 balance) {
            string memory symbol = getSymbolSafe(tokenContract);
            uint8 decimals = getDecimalsSafe(tokenContract);
            
            console.log(" Token Balance Check");
            console.log("==================================================");
            console.log("Token:", token);
            console.log("Symbol:", symbol);
            console.log("Decimals:", decimals);
            console.log("Address:", account);
            console.log("Balance:", formatTokenBalance(balance, decimals), symbol);
            console.log("Raw balance:", balance);
        } catch {
            console.log(" Error: Could not read token balance");
        }
    }
    
    function checkAllowance() internal view {
        address token = vm.envAddress("TOKEN_ADDRESS");
        address owner = vm.envAddress("OWNER_ADDRESS");
        address spender = vm.envAddress("SPENDER_ADDRESS");
        
        IERC20 tokenContract = getTokenContract(token);
        
        try tokenContract.allowance(owner, spender) returns (uint256 allowance) {
            string memory symbol = getSymbolSafe(tokenContract);
            
            console.log(" Token Allowance Check");
            console.log("==================================================");
            console.log("Token:", token);
            console.log("Symbol:", symbol);
            console.log("Owner:", owner);
            console.log("Spender:", spender);
            console.log("Allowance:", formatTokenBalance(allowance, getDecimalsSafe(tokenContract)), symbol);
            console.log("Raw allowance:", allowance);
        } catch {
            console.log(" Error: Could not read token allowance");
        }
    }
    
    function approveToken() internal {
        address token = vm.envAddress("TOKEN_ADDRESS");
        address spender = vm.envAddress("SPENDER_ADDRESS");
        uint256 amount = vm.envUint("AMOUNT");
        
        IERC20 tokenContract = getTokenContract(token);
        
        string memory symbol = getSymbolSafe(tokenContract);
        uint8 decimals = getDecimalsSafe(tokenContract);
        
        console.log(" Setting Token Allowance");
        console.log("==================================================");
        console.log("Token:", token);
        console.log("Symbol:", symbol);
        console.log("Decimals:", decimals);
        console.log("Spender:", spender);
        console.log("Amount:", amount, symbol);
        
        // Parse amount using correct decimals
        uint256 parsedAmount = amount * (10 ** decimals);
        console.log("Raw amount:", parsedAmount);
        console.log("Decimals:", decimals);
        
        vm.startBroadcast();
        
        try tokenContract.approve(spender, parsedAmount) returns (bool success) {
            if (success) {
                console.log(" Approval transaction sent");
                console.log("Transaction completed");
                
                // Verify the allowance was set correctly
                uint256 newAllowance = tokenContract.allowance(msg.sender, spender);
                console.log(" New allowance:", formatTokenBalance(newAllowance, decimals), symbol);
            } else {
                console.log(" Approval transaction failed");
            }
        } catch Error(string memory reason) {
            console.log(" Error:", reason);
        } catch {
            console.log(" Error: Approval failed");
        }
        
        vm.stopBroadcast();
    }
    
    function getSymbolSafe(IERC20 token) internal view returns (string memory) {
        try token.symbol() returns (string memory symbol) {
            return symbol;
        } catch {
            return "UNKNOWN";
        }
    }
    
    function getDecimalsSafe(IERC20 token) internal view returns (uint8) {
        try token.decimals() returns (uint8 decimals) {
            return decimals;
        } catch {
            return 18;
        }
    }
    
    function formatTokenBalance(uint256 balance, uint8 decimals) internal pure returns (string memory) {
        if (decimals == 0) return vm.toString(balance);
        
        uint256 divisor = 10 ** decimals;
        uint256 wholePart = balance / divisor;
        uint256 fractionalPart = balance % divisor;
        
        if (fractionalPart == 0) {
            return vm.toString(wholePart);
        }
        
        // Format with decimal point
        string memory fractionalStr = vm.toString(fractionalPart);
        
        // Pad with leading zeros if necessary
        bytes memory fractionalBytes = bytes(fractionalStr);
        uint256 zerosNeeded = decimals - fractionalBytes.length;
        
        string memory paddedFractional = fractionalStr;
        for (uint256 i = 0; i < zerosNeeded; i++) {
            paddedFractional = string(abi.encodePacked("0", paddedFractional));
        }
        
        // Remove trailing zeros
        bytes memory paddedBytes = bytes(paddedFractional);
        uint256 trimIndex = paddedBytes.length;
        while (trimIndex > 0 && paddedBytes[trimIndex - 1] == '0') {
            trimIndex--;
        }
        
        if (trimIndex == 0) {
            return vm.toString(wholePart);
        }
        
        bytes memory trimmedBytes = new bytes(trimIndex);
        for (uint256 i = 0; i < trimIndex; i++) {
            trimmedBytes[i] = paddedBytes[i];
        }
        
        return string(abi.encodePacked(vm.toString(wholePart), ".", string(trimmedBytes)));
    }
    
    // Helper function to get token contract with proper detection
    function getTokenContract(address tokenAddress) internal view returns (IERC20) {
        // In Foundry, we don't have the luxury of trying different contract names like in Hardhat
        // Instead, we use the standard IERC20 interface and test if the contract responds correctly
        IERC20 tokenContract = IERC20(tokenAddress);
        
        try tokenContract.name() returns (string memory) {
            try tokenContract.symbol() returns (string memory) {
                try tokenContract.decimals() returns (uint8) {
                    console.log(" Using standard ERC20 interface");
                    return tokenContract;
                } catch {
                    revert("Token contract does not implement decimals()");
                }
            } catch {
                revert("Token contract does not implement symbol()");
            }
        } catch {
            revert("Token contract does not implement name()");
        }
    }
}
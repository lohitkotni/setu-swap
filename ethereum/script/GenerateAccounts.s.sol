// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {StdCheats} from "forge-std/StdCheats.sol";

contract GenerateAccounts is Script {
    function run() external {
        string memory mnemonic = "test test test test test test test test test test test junk";
        
        for (uint32 i = 0; i < 20; i++) {
            // Derive the private key for index i
            (address account, uint256 privateKey)  = deriveRememberKey(mnemonic, i);
            
            console.log("Account %d    : %s", i, account);
            console.log("Private Key %d: 0x%064x", i, privateKey);
        }
    }
}
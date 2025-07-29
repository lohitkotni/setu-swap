// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// 1. Import Foundry’s Script base
import "forge-std/Script.sol";

// 2. Import your contract
import "../src/MinimalEscrowFactory.sol";

// 3. Define the deploy script contract
contract DeployMinimalEscrow is Script {
    function run() external {
        // 4. Get the private key context
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        // 5. Start broadcasting (transaction sending)
        vm.startBroadcast(deployerPrivateKey);

        // 6. Deploy the contract with desired constructor args
        MinimalEscrowFactory factory = new MinimalEscrowFactory(
            3600,  // RESCUE_DELAY_SRC
            7200   // RESCUE_DELAY_DST
        );

        // 7. Stop broadcasting
        vm.stopBroadcast();

        // 8. (Optional) log the address
        console.log("Deployed MinimalEscrowFactory to:", address(factory));
    }
}

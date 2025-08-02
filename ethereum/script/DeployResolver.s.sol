// script/DeployResolver.s.sol
// SPDX-License-Identifier: MIT
pragma solidity 0.8.23;

import "forge-std/Script.sol";
import "../src/Resolver.sol";
import "../src/interfaces/IEscrowFactory.sol";
import "lib/limit-order-protocol/contracts/interfaces/IOrderMixin.sol";

contract DeployResolver is Script {
    function run() external {
        // Get private key from environment
        uint256 deployerPrivateKey = 0x668c80e625b3281a42fdf0aec0f9b10da24b1197f7545d452b53780150130800;
        
        // Get configuration from environment
        address escrowFactory = 0x4aE1640FF096c843e9B229993f34c9Fe84B3Aa52;  // existing factory address
        address limitOrderProtocol = 0xc0ffee254729296a45a3885639AC7E10F9d54979;
        address owner = msg.sender;

        vm.startBroadcast(deployerPrivateKey);

        // Deploy Resolver using existing factory
        Resolver resolver = new Resolver(
            IEscrowFactory(escrowFactory),
            IOrderMixin(limitOrderProtocol),
            owner
        );

        vm.stopBroadcast();

        // Log the deployed address
        console.log("Resolver deployed at:", address(resolver));
    }
}
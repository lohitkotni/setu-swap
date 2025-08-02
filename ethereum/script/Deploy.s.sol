// script/DeployEscrow.s.sol
// SPDX-License-Identifier: MIT
pragma solidity 0.8.23;

import "forge-std/Script.sol";
import "../src/EscrowFactory.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract DeployEscrow is Script {
    function run() external {
        // Get private key from environment
        uint256 deployerPrivateKey = 0x668c80e625b3281a42fdf0aec0f9b10da24b1197f7545d452b53780150130800;
        
        // Get configuration from environment
        address limitOrderProtocol = 0xc0ffee254729296a45a3885639AC7E10F9d54979;
        address feeToken = 0x111111111117dC0aa78b770fA6A738034120C302;
        address accessToken = 0x111111111117dC0aa78b770fA6A738034120C302;
        address owner = msg.sender;
        uint32 rescueDelaySrc = 24 * 60 * 60;
        uint32 rescueDelayDst = 24 * 60 * 60;

        vm.startBroadcast(deployerPrivateKey);

        // Deploy EscrowFactory
        EscrowFactory factory = new EscrowFactory(
            limitOrderProtocol,
            IERC20(feeToken),
            IERC20(accessToken),
            owner,
            rescueDelaySrc,
            rescueDelayDst
        );

        vm.stopBroadcast();

        // Log the deployed addresses
        console.log("EscrowFactory deployed at:", address(factory));
    }
}
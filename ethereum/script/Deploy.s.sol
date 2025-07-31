// SPDX-License-Identifier: MIT

pragma solidity ^0.8.23;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {EscrowFactory} from "../src/EscrowFactory.sol";
import {IERC20} from "lib/openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

contract Deploy is Script {
    function run() external {
        console.log("Deploying EscrowFactory...");
        
        address deployer = msg.sender;
        console.log("Deployer:", deployer);

        vm.startBroadcast();
        console.log("access token deployed to:", address(0xc7657D8c30c9C3792C9697c9A2F772F41cCe19c5));

        console.log("stable token deployed to:", address(0xC7b69070352f04d5aAa33eda4916EdE802db0edf));

        console.log("meme token deployed to:", address(0xf0E43392020D5B8db4E691aD16642Ee777f818B9));

        address limitOrderProtocol = address(0x0000000000000000000000000000000000000000);
        IERC20 feeToken = IERC20(address(0xc7657D8c30c9C3792C9697c9A2F772F41cCe19c5));
        IERC20 accessToken = IERC20(address(0xf0E43392020D5B8db4E691aD16642Ee777f818B9));
        address owner = deployer;
        uint32 rescueDelaySrc = 24 * 60 * 60;
        uint32 rescueDelayDst = 24 * 60 * 60;

        EscrowFactory factory = new EscrowFactory(limitOrderProtocol, feeToken, accessToken, owner, rescueDelaySrc, rescueDelayDst);

        console.log("EscrowFactory deployed to:", address(factory));

        address srcImplementation = factory.ESCROW_SRC_IMPLEMENTATION();
        console.log("srcImplementation:", srcImplementation);

        address dstImplementation = factory.ESCROW_DST_IMPLEMENTATION();
        console.log("dstImplementation:", dstImplementation);

        vm.stopBroadcast();
    }
}
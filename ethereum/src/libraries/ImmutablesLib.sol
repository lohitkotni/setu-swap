// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;
import { AddressLib, Address } from "@1inch/libraries/AddressLib.sol";
import { Timelocks, TimelocksLib } from "./TimelocksLib.sol";


/**
 * @title Library for escrow immutables.
 * @custom:security-contact security@1inch.io
 */

    struct Immutables {
        bytes32 orderHash;
        bytes32 hashlock;  // Hash of the secret.
        Address maker;
        Address taker;
        Address token;
        uint256 amount;
        uint256 safetyDeposit;
        Timelocks timelocks;
    }
library ImmutablesLib {
    uint256 internal constant ESCROW_IMMUTABLES_SIZE = 0x100;
     
    /**
     * @notice Returns the hash of the immutables.
     * @param immutables The immutables to hash.
     * @return ret The computed hash.
     */
    function hash(Immutables calldata immutables) internal pure returns(bytes32 ret) {
        assembly ("memory-safe") {
            let ptr := mload(0x40)
            calldatacopy(ptr, immutables, ESCROW_IMMUTABLES_SIZE)
            ret := keccak256(ptr, ESCROW_IMMUTABLES_SIZE)
        }
    }

    /**
     * @notice Returns the hash of the immutables.
     * @param immutables The immutables to hash.
     * @return ret The computed hash.
     */
    function hashMem(Immutables memory immutables) internal pure returns(bytes32 ret) {
        assembly ("memory-safe") {
            ret := keccak256(immutables, ESCROW_IMMUTABLES_SIZE)
        }
    }
}
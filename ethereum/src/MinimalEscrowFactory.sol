// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { Address, AddressLib } from "@1inch/libraries/AddressLib.sol";
import { SafeERC20 } from "@1inch/libraries/SafeERC20.sol";

import { ImmutablesLib, Immutables } from "./libraries/ImmutablesLib.sol";
import { Timelocks, TimelocksLib } from "./libraries/TimelocksLib.sol";
import { MinimalEscrowSrc } from "./MinimalEscrowSrc.sol";
import { MinimalEscrowDst } from "./MinimalEscrowDst.sol";

/**
 * @title Minimal Escrow Factory contract
 * @notice Simple factory to create escrow contracts for cross-chain atomic swap.
 */
contract MinimalEscrowFactory {
    using AddressLib for Address;
    using ImmutablesLib for Immutables;
    using SafeERC20 for IERC20;
    using TimelocksLib for Timelocks;

    uint32 public immutable rescueDelaySrc;
    uint32 public immutable rescueDelayDst;

    error InsufficientEscrowBalance();
    error InvalidCreationTime();

    /**
     * @notice Emitted on source escrow deployment.
     * @param escrow The address of the created escrow.
     * @param hashlock The hash of the secret.
     * @param maker The address of the maker.
     * @param taker The address of the taker.
     */
    event SrcEscrowCreated(address escrow, bytes32 hashlock, address maker, address taker);
    
    /**
     * @notice Emitted on destination escrow deployment.
     * @param escrow The address of the created escrow.
     * @param hashlock The hash of the secret.
     * @param taker The address of the taker.
     */
    event DstEscrowCreated(address escrow, bytes32 hashlock, Address taker);

    constructor(uint32 rescueDelaySrc_, uint32 rescueDelayDst_) {
        rescueDelaySrc = rescueDelaySrc_;
        rescueDelayDst = rescueDelayDst_;
    }

    /**
     * @notice Creates a new escrow contract for the source chain.
     * @param immutables The immutables of the escrow contract.
     */
    function createSrcEscrow(Immutables calldata immutables) external payable {
        address token = immutables.token.get();
        uint256 nativeAmount = immutables.safetyDeposit;
        if (token == address(0)) {
            nativeAmount += immutables.amount;
        }
        if (msg.value != nativeAmount) revert InsufficientEscrowBalance();

        Immutables memory srcImmutables = immutables;
        srcImmutables.timelocks = srcImmutables.timelocks.setDeployedAt(block.timestamp);

        MinimalEscrowSrc escrow = new MinimalEscrowSrc(uint32(rescueDelaySrc), IERC20(token));
        
        if (token != address(0)) {
            IERC20(token).safeTransferFrom(msg.sender, address(escrow), immutables.amount);
        }

        emit SrcEscrowCreated(address(escrow), immutables.hashlock, immutables.maker.get(), immutables.taker.get());
    }

    /**
     * @notice Creates a new escrow contract for the destination chain.
     * @param dstImmutables The immutables of the escrow contract.
     * @param srcCancellationTimestamp The start of the cancellation period for the source chain.
     */
    function createDstEscrow(Immutables calldata dstImmutables, uint256 srcCancellationTimestamp) external payable {
        address token = dstImmutables.token.get();
        uint256 nativeAmount = dstImmutables.safetyDeposit;
        if (token == address(0)) {
            nativeAmount += dstImmutables.amount;
        }
        if (msg.value != nativeAmount) revert InsufficientEscrowBalance();

        Immutables memory immutables = dstImmutables;
        immutables.timelocks = immutables.timelocks.setDeployedAt(block.timestamp);
        
        // Check that the escrow cancellation will start not later than the cancellation time on the source chain.
        if (immutables.timelocks.get(TimelocksLib.Stage.DstCancellation) > srcCancellationTimestamp) revert InvalidCreationTime();

        MinimalEscrowDst escrow = new MinimalEscrowDst(uint32(rescueDelayDst), IERC20(token));
        
        if (token != address(0)) {
            IERC20(token).safeTransferFrom(msg.sender, address(escrow), immutables.amount);
        }

        emit DstEscrowCreated(address(escrow), dstImmutables.hashlock, dstImmutables.taker);
    }
} 
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/MinimalEscrowFactory.sol";
import "./mocks/MockToken.sol";
import "../src/interfaces/IBaseEscrow.sol";
import { Address } from "@1inch/libraries/AddressLib.sol";

contract MinimalEscrowFactoryTest is Test {
    MinimalEscrowFactory factory;
    MockToken token;

    address maker;
    address taker;

    uint32 constant RESCUE_DELAY_SRC = 3600; // 1 hour
    uint32 constant RESCUE_DELAY_DST = 7200; // 2 hours

    function setUp() public {
        maker = vm.addr(1);
        taker = vm.addr(2);

        vm.deal(maker, 10 ether);
        vm.deal(taker, 10 ether);

        factory = new MinimalEscrowFactory(RESCUE_DELAY_SRC, RESCUE_DELAY_DST);
        token = new MockToken("Test Token", "TST");
    }

    function testCreateDstEscrow() public {
        uint256 amount = 100 ether;
        uint256 safetyDeposit = 1 ether;

        vm.prank(address(this)); // mint to taker
        token.mint(taker, amount);

        vm.prank(taker);
        token.approve(address(factory), amount);

        bytes32 orderHash = keccak256("orderHash");
        bytes32 hashlock = keccak256("hashlock");
        uint256 srcCancellationTimestamp = block.timestamp + 3600;

        uint256 dstWithdrawalDelay = 100;
        uint256 dstPublicWithdrawalDelay = 200;
        uint256 dstCancellationDelay = 300;

        uint256 packedTimelocks = 0;
        packedTimelocks |= dstWithdrawalDelay << (4 * 32);
        packedTimelocks |= dstPublicWithdrawalDelay << (5 * 32);
        packedTimelocks |= dstCancellationDelay << (6 * 32);

        IBaseEscrow.Immutables memory dstImmutables = IBaseEscrow.Immutables({
            orderHash: orderHash,
            hashlock: hashlock,
            maker: Address.wrap(uint256(uint160(maker))),
            taker: Address.wrap(uint256(uint160(taker))),
            token: Address.wrap(uint256(uint160(address(token)))),
            amount: amount,
            safetyDeposit: safetyDeposit,
            timelocks: Timelocks.wrap(packedTimelocks)
        });

        vm.prank(taker);
        vm.recordLogs();
        factory.createDstEscrow{value: safetyDeposit}(dstImmutables, srcCancellationTimestamp);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        address escrowAddress = address(0);
        bytes32 expectedEventSig = keccak256("DstEscrowCreated(address,bytes32,address)");

        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == expectedEventSig) {
                escrowAddress = address(uint160(uint256(logs[i].topics[1])));
                break;
            }
        }

        assertTrue(escrowAddress != address(0), "DstEscrowCreated not emitted");

        assertEq(token.balanceOf(escrowAddress), amount, "Incorrect token balance in escrow");
        assertEq(escrowAddress.balance, safetyDeposit, "Incorrect ETH balance in escrow");

        uint256 expectedCancellation = block.timestamp + dstCancellationDelay;
        assertLe(expectedCancellation, srcCancellationTimestamp);
    }
}

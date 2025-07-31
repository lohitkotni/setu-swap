// SPDX-License-Identifier: MIT

pragma solidity 0.8.23;

import { IERC20 } from "lib/openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import { IOrderMixin } from "lib/limit-order-protocol/contracts/interfaces/IOrderMixin.sol";

/**
 * @title Resolver Validation Extension
 * @notice This abstract contract combines functionalities to enhance security and compliance in the order execution process.
 * Ensures that only transactions from whitelisted resolvers or resolvers who own specific accessToken are processed within the post-interaction phase of order execution.
 * Additionally, it allows charging a fee to resolvers in the `postInteraction` method, providing a mechanism for resolver fee management.
 */
abstract contract ResolverValidationExtension {

    error ResolverCanNotFillOrder();

    /// @notice Contract address whose tokens allow filling limit orders with a fee for resolvers that are outside the whitelist
    IERC20 private immutable _ACCESS_TOKEN;
    uint256 private constant _ORDER_FEE_BASE_POINTS = 1e15;
    IERC20 private immutable _FEE_TOKEN;
    address private immutable _OWNER;

    constructor(IERC20 feeToken, IERC20 accessToken, address owner) {
        _ACCESS_TOKEN = accessToken;
        _FEE_TOKEN = feeToken;
        _OWNER = owner;
    }
    
    /**
     * @dev Calculates the resolver fee.
     * @param fee Scaled resolver fee.
     * @param orderMakingAmount Making amount from the order.
     * @param actualMakingAmount Making amount that was actually filled.
     * @return resolverFee Calculated resolver fee.
     */
    function _getResolverFee(
        uint256 fee,
        uint256 orderMakingAmount,
        uint256 actualMakingAmount
    ) internal pure virtual returns(uint256) {
        return fee * _ORDER_FEE_BASE_POINTS * actualMakingAmount / orderMakingAmount;
    }

    
    function _postInteraction(
        IOrderMixin.Order calldata order,
        bytes calldata extension,
        bytes32 orderHash,
        address taker,
        uint256 makingAmount,
        uint256 takingAmount,
        uint256 remainingMakingAmount,
        bytes calldata extraData
    ) internal virtual {

    }
}
// SPDX-License-Identifier: MIT

pragma solidity 0.8.23;

import { ERC20 } from "lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract WETH is ERC20 {
    uint256 public constant TOTAL_SUPPLY = 1000000000000000000000000000;
    constructor() ERC20("Wrapped Ethereum", "WETH") {
        _mint(msg.sender, TOTAL_SUPPLY);
    }
}
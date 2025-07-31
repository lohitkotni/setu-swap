// SPDX-License-Identifier: MIT

pragma solidity 0.8.23;

import { ERC20 } from "lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract AccessToken is ERC20 {
    uint256 private constant _ACCESS_TOKEN_SUPPLY = 1000000000000000000000000000;
    constructor() ERC20("AccessToken", "ATK") {
        _mint(msg.sender, _ACCESS_TOKEN_SUPPLY);
    }
}
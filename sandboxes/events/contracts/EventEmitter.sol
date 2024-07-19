// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract EventEmitter {
    event ValueChanged(uint indexed newValue);

    function setValue(uint _value) external {
        emit ValueChanged(_value);
    }
}
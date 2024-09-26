// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Script.sol";
import "./MockEvent.sol";
import "lib/openzeppelin-contracts/contracts/utils/Strings.sol";  // Import Strings library

contract CallDispatch is Script {
    MockEvent public contractInstance;

    function run() external {
        address contractAddress = 0x7a2088a1bfc9d81c55368ae168c2c02570cb814f;
        contractInstance = MockEvent(contractAddress);

        uint256 count = 0;
        while (true) {
            string memory message = string(abi.encodePacked("Message ", Strings.toString(count)));
            contractInstance.dispatchEvent(message);
            console.log("Dispatched event with message:", message);
            count++;

            // Wait a bit before sending the next message
            vm.warp(block.timestamp + 1); // fast-forward time by 1 second
        }
    }
}

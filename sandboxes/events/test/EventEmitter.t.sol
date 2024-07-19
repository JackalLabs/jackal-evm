// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../contracts/EventEmitter.sol";

contract EventEmitterTest is Test {
    EventEmitter public emitter;

    function setUp() public {
        emitter = new EventEmitter();
    }

    function testSetValue() public {
        uint expectedValue = 123;

        // Setup the expected emission of the ValueChanged event
        vm.expectEmit(true, false, false, false);
        // Note: Correct usage is directly emitting the event with expected values for testing
        emitter.setValue(expectedValue);

        // Check that the expected event was actually emitted
        // This is usually handled by expectEmit and a corresponding assertion is not needed in most cases
        // However, the assertion below is to visually confirm in test outputs or additional validations if required
        assertEq(emitter.ValueChanged(expectedValue), expectedValue);
    }
}

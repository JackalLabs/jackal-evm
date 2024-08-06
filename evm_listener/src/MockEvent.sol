pragma solidity ^0.8.25;

contract MockEvent {
    event Dispatch(address indexed sender, string value, string message);

    function dispatchEvent(string memory value) public {
        emit Dispatch(msg.sender, value, "Dispatch bytes received!");
    }
} 
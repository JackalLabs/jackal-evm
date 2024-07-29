pragma solidity ^0.8.26;

contract TestEvent {
    event Dispatch(address indexed sender, uint256 value, string message);

    function dispatchEvent(uint256 value) public {
        emit Dispatch(msg.sender, value, "Dispatch bytes received!");
    }
}
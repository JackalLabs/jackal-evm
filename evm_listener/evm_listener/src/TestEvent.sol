pragma solidity ^0.8.26;

contract TestEvent {
    event Dispatch(address indexed sender, string message);

    function dispatchEvent() public {
        emit Dispatch(msg.sender, "Dispatch bytes received!");
    }
}
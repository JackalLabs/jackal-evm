// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

contract PiggyBankEvents {
    event Deposited(address indexed from, address indexed to, uint256 amount);
    event Withdrawn(address indexed to, uint256 amount);
}

contract PiggyBank is PiggyBankEvents {
    uint256 public totalBalance;
    mapping(address account => uint256) public balances;

    function deposit(address _account) external payable {
        require(msg.value != 0, "invalid deposit");

        // Increment record
        totalBalance += msg.value;
        balances[_account] += msg.value;

        // Emit event
        emit Deposited(msg.sender, _account, msg.value);
    }

    function withdraw(uint256 _amount) external {
        require(balances[msg.sender] >= _amount, "balance too low");

        // Decrement record
        totalBalance -= _amount;
        balances[msg.sender] -= _amount;

        payable(msg.sender).transfer(_amount);

        // Emit event
        emit Withdrawn(msg.sender, _amount);
    }
}
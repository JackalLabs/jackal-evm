// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test, Vm} from "forge-std/Test.sol";
import {PiggyBank, PiggyBankEvents} from "../src/PiggyBank.sol";

contract PiggyBankTest is Test, PiggyBankEvents {
    address internal constant RECEIVER = 
        address(uint160(uint256(keccak256("piggy bank test receiver"))));

    function setUp() public {
        vm.label(msg.sender, "MSG_SENDER");
        vm.label(RECEIVER, "RECEIVER");
    }

    function testPiggyBank_Withdraw() public {
        // Create PiggyBank contract
        PiggyBank piggyBank = new PiggyBank();
        uint256 _amount = 1000;

        // Deposit
        vm.deal(msg.sender, _amount);
        vm.startPrank(msg.sender);
        (bool _success, ) = address(piggyBank).call{value: _amount}(
            abi.encodeWithSignature("deposit(address)", msg.sender)
        );
        assertTrue(_success, "deposited payment.");
        vm.stopPrank();

        // Set withdraw event expectations
        vm.expectEmit(true, false, false, true, address(piggyBank));
        emit Withdrawn(msg.sender, 1000);
        
        // Withdraw
        vm.startPrank(msg.sender);
        piggyBank.withdraw(_amount);
        vm.stopPrank();




    }


}
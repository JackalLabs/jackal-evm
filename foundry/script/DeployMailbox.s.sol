// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import "../solidity/contracts/test/TestMailbox.sol";
import "forge-std/console.sol"; // Add this import

import "../solidity/contracts/test/TestIsm.sol";

import "../solidity/contracts/test/TestRecipient.sol";
import "../solidity/contracts/test/TestPostDispatchHook.sol";
import {StandardHookMetadata} from "../solidity/contracts/hooks/libs/StandardHookMetadata.sol";
import "../solidity/contracts/libs/TypeCasts.sol";

contract MailboxScript is Script {

    using StandardHookMetadata for bytes;
    using TypeCasts for address;
    using Message for bytes;

    // NOTE: Can the 'Test' contracts be deployed on anvil? 
    TestMailbox mailbox;
    TestRecipient recipient;
    bytes32 recipientb32;

    uint32 localDomain = 1;
    uint32 remoteDomain = 2; // so the domain of the mailbox in wasmvm has to be 2?

    TestPostDispatchHook defaultHook;
    TestPostDispatchHook overrideHook;
    TestPostDispatchHook requiredHook;
    TestIsm defaultIsm;

    address owner;

    function setUp() public {
        address anvilAccount = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;

        mailbox = new TestMailbox(localDomain);
        recipient = new TestRecipient(); 
        recipientb32 = address(recipient).addressToBytes32();

        defaultHook = new TestPostDispatchHook();
        // Uncomment and initialize if needed
        // merkleHook = new MerkleTreeHook(address(mailbox));
        requiredHook = new TestPostDispatchHook();
        overrideHook = new TestPostDispatchHook();
        defaultIsm = new TestIsm();  

        owner = msg.sender;

        // Without 'prank()', the caller of initialize would actually be the address of 'MailboxScript' instead of 'owner'
        vm.prank(anvilAccount);
        mailbox.initialize(
            anvilAccount,
            address(defaultIsm),
            address(defaultHook),
            address(requiredHook)
        );

        // One of these hooks is not being called, let's try and figure out which one 
        console.log("defaultHook is:", address(defaultHook));
        console.log("requiredHook is:", address(requiredHook));

        // NOTE: Upon logging, we see that 'defaultHook' is not being called 

        // Verify ownership
        address mailboxOwner = mailbox.owner();
        require(mailboxOwner == anvilAccount, "Owner not set correctly");
 
    }

    function run() public {
        address anvilAccount = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;

        uint256 deployerPrivateKey;
        deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);
        // Make the next transaction come from anvilAccount

        // don't need prefixedMetadata yet 

        bytes memory metadataPlaceholder = new bytes(10);
        bytes memory body = new bytes(10);

        uint256 quote;
        uint32 nonce;
        bytes32 id;

        // This calls TestPostDispatchHook which just sets a hard coded fee for now
        quote = mailbox.quoteDispatch(remoteDomain, recipientb32, body);
        console.log("The quote is:", quote);

        expectDispatch(anvilAccount,requiredHook, defaultHook, metadataPlaceholder, body);
        id = mailbox.dispatch{value: 1000}(
            remoteDomain,
            recipientb32,
            body
            // NOTE: we didn't put metadata in here so it will be "0x" (empty metadata)
        );
        vm.stopBroadcast();
    }

    function expectDispatch(
        address dispatcher,
        TestPostDispatchHook firstHook,
        TestPostDispatchHook hook,
        bytes memory metadata,
        bytes memory body
    ) internal {
        bytes memory message = mailbox.buildOutboundMessage(
            remoteDomain,
            recipientb32,
            body
        );
        vm.expectEmit(true, true, true, true, address(mailbox));
        console.log("The address we expected to be the dispatcher is:", msg.sender);
        console.log("Balance of msg.sender:", address(msg.sender).balance);
        emit Dispatch(dispatcher, remoteDomain, recipientb32, message); 
        // NOTE: we accidentally made the sender of dispatch the address of this contract--address(this)--instead of the test runner--msg.sender
    }

    event Dispatch(
        address indexed sender,
        uint32 indexed destination,
        bytes32 indexed recipient,
        bytes message
    );

    event DispatchId(bytes32 indexed messageId);

    function bytesToHexString(bytes32 data) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(64); // Length of a bytes32 * 2
        for (uint i = 0; i < 32; i++) {
            str[i*2] = alphabet[uint(uint8(data[i] >> 4))];
            str[1+i*2] = alphabet[uint(uint8(data[i] & 0x0f))];
        }
        return string(str);
    }

}

// NOTE: Let's peer it down even more to really isolate
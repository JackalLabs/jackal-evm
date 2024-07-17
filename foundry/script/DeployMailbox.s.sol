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
        vm.prank(owner);
        mailbox.initialize(
            owner,
            address(defaultIsm),
            address(defaultHook),
            address(requiredHook)
        );

        // Verify ownership
        address mailboxOwner = mailbox.owner();
        require(mailboxOwner == owner, "Owner not set correctly");
 
    }

    function run() public {
        vm.startBroadcast();

        // // I don't think we're going to use a hook for now 
        // bytes memory prefixedMetadata = abi.encodePacked(
        //     StandardHookMetadata.VARIANT,
        //     metadata
        // );
        bytes memory metadataPlaceholder = new bytes(10);
        bytes memory body = new bytes(10);

        uint256 quote;

        // This calls TestPostDispatchHook which just sets a hard coded fee for now
        quote = mailbox.quoteDispatch(remoteDomain, recipientb32, body);
        expectDispatch(requiredHook, defaultHook, metadataPlaceholder, body);


        vm.stopBroadcast();
    }

    function expectDispatch(
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
        expectHookQuote(firstHook, metadata, message);
        expectHookPost(firstHook, metadata, message, firstHook.fee());
        expectHookPost(hook, metadata, message, hook.fee());
        vm.expectEmit(true, true, true, true, address(mailbox));
        emit Dispatch(address(this), remoteDomain, recipientb32, message);
        vm.expectEmit(true, false, false, false, address(mailbox));
        emit DispatchId(message.id());
    }

    function expectHookQuote(
        IPostDispatchHook hook,
        bytes memory metadata,
        bytes memory message
    ) internal {
        vm.expectCall(
            address(hook),
            abi.encodeCall(IPostDispatchHook.quoteDispatch, (metadata, message))
        );
    }

    function expectHookPost(
        IPostDispatchHook hook,
        bytes memory metadata,
        bytes memory message,
        uint256 value
    ) internal {
        vm.expectCall(
            address(hook),
            value,
            abi.encodeCall(IPostDispatchHook.postDispatch, (metadata, message))
        );
    }

    event Dispatch(
        address indexed sender,
        uint32 indexed destination,
        bytes32 indexed recipient,
        bytes message
    );

    event DispatchId(bytes32 indexed messageId);

}

// NOTE: Let's peer it down even more to really isolate
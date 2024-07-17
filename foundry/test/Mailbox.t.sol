// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console.sol"; // Add this import
import "../solidity/contracts/test/TestMailbox.sol";
import "../solidity/contracts/test/TestIsm.sol";
import "../solidity/contracts/test/TestRecipient.sol";
import "../solidity/contracts/test/TestPostDispatchHook.sol";
import {StandardHookMetadata} from "../solidity/contracts/hooks/libs/StandardHookMetadata.sol";

contract Empty {}

contract EmptyFallback {
    fallback() external {}
}

contract mailboxTest is Test {
    using StandardHookMetadata for bytes;
    using TypeCasts for address;
    using Message for bytes;

    uint32 localDomain = 1;
    uint32 remoteDomain = 2; // so the domain of the mailbox in wasmvm has to be 2?
    TestMailbox mailbox;

    // MerkleTreeHook merkleHook; *NOTE: really curious if the test passes without this hook

    TestPostDispatchHook defaultHook;
    TestPostDispatchHook overrideHook;
    TestPostDispatchHook requiredHook;

    TestIsm defaultIsm;
    TestRecipient recipient;
    bytes32 recipientb32;

    address owner;

    function setUp() public {
        mailbox = new TestMailbox(localDomain);
        recipient = new TestRecipient(); // Initialize TestRecipient correctly
        recipientb32 = address(recipient).addressToBytes32();
        defaultHook = new TestPostDispatchHook();
        // Uncomment and initialize if needed
        // merkleHook = new MerkleTreeHook(address(mailbox));
        requiredHook = new TestPostDispatchHook();
        overrideHook = new TestPostDispatchHook();
        defaultIsm = new TestIsm();     
   
        owner = msg.sender;

        // Before adding the below line, the address that called 'mailbox.initialize' was actually the address of the mailboxTest contract.
        // To properly initialize with ownership, the address that calls mailbox.initalize needs to be one and the same as the passed in owner
        // which we want to be 'msg.sender'.
        // We use 'vm.prank(owner)' to ensure that msg.sender (the test runner) is making the mailbox.initialize call--NOT the mailboxTest contract.
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

    // function test_localDomain() public {
    //     assertEq(mailbox.localDomain(), localDomain);
    // }

    // function test_initialize() public {
    //     assertEq(mailbox.owner(), owner);
    //     assertEq(address(mailbox.defaultIsm()), address(defaultIsm));
    //     assertEq(address(mailbox.defaultHook()), address(defaultHook));
    //     assertEq(address(mailbox.requiredHook()), address(requiredHook));
    // }

// NOTE: a test has to have an assertion otherwise 'forge test' will skip all the logic inside the function
// even though you are notified in the terminal that the test is run
    function test_dispatch(
        uint8 n,
        bytes calldata body, // seems like the test runner isn't really passing anything into here?
        bytes calldata metadata
    ) public {
        console.log("Starting test_dispatch");

        // I don't think we're going to use a hook for now 
        bytes memory prefixedMetadata = abi.encodePacked(
            StandardHookMetadata.VARIANT,
            metadata
        );
        bytes calldata defaultMetadata = metadata[0:0];

        console.log("default metadata is:", bytesToHexString(defaultMetadata));

        uint256 quote;
        uint32 nonce;
        bytes32 id;

        bytes memory largeBytes = new bytes(5120000);

        //WARNING: forge logging does not support bytes 
        console.log("lenth of largeBytes is:", largeBytes.length);
        console.log("lenth of body is:", body.length);

        console.log("Input parameter n:", n);

        // hyperlane mono-repo tests 3 dispatches and checks to make sure the nonce increasese each time
        // we're skipping that for now

        // NOTE: so we took out this loop and it appeared that the test passed. But it seems that taking out the loop
        // and not using 'n' cause this function to be bypassed completely? 

        for (uint256 i = 0; i < n; i += 3) {
            nonce = mailbox.nonce();

            // The TestPostDispatchHook just sets the quote to 0, this isn't really practical for 
            // estimating the gas cost on testnet and main net 
            quote = mailbox.quoteDispatch(remoteDomain, recipientb32, largeBytes);
            expectDispatch(requiredHook, defaultHook, defaultMetadata, body);
            id = mailbox.dispatch{value: quote}(
                remoteDomain,
                recipientb32, 
                body
            );
            assertEq(mailbox.latestDispatchedId(), id);
            console.log("the quote is:", quote);

            // assertEq(nonce, i); 

        }

    }

    function bytesToHexString(bytes32 data) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(64); // Length of a bytes32 * 2
        for (uint i = 0; i < 32; i++) {
            str[i*2] = alphabet[uint(uint8(data[i] >> 4))];
            str[1+i*2] = alphabet[uint(uint8(data[i] & 0x0f))];
        }
        return string(str);
    }

    function bytesToHexString(bytes calldata data) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(2 + data.length * 2);
        str[0] = '0';
        str[1] = 'x';
        for (uint i = 0; i < data.length; i++) {
            str[2+i*2] = alphabet[uint(uint8(data[i] >> 4))];
            str[3+i*2] = alphabet[uint(uint8(data[i] & 0x0f))];
        }
        return string(str);
    }

    function expectDispatch(
        TestPostDispatchHook firstHook,
        TestPostDispatchHook hook,
        bytes memory metadata,
        bytes calldata body
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

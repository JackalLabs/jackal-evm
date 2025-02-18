package interchaintest

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"strconv"
	"time"

	testsuite "github.com/JackalLabs/jackal-evm/testsuite"
	icatypes "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/types"

	allbindingstypes "github.com/JackalLabs/jackal-evm/types/bindings"
	factorytypes "github.com/JackalLabs/jackal-evm/types/bindingsfactory"

	logger "github.com/JackalLabs/storage-outpost/e2e/interchaintest/logger"
)

// WARNING: strangelove's test package builds chains running ibc-go/v7
// Hopefully this won't cause issues because the canined image we use is running ibc-go/v4
// and packets should be consumed by the ica host no matter what version of ibc-go the controller chain is running

// Testing canine-chain's web assembly bindings
func (s *ContractTestSuite) TestNotificationsModule() {
	ctx := context.Background()

	logger.InitLogger()

	encoding := icatypes.EncodingProtobuf
	// This starts the chains, relayer, creates the user accounts, creates the ibc clients and connections,
	// sets up the contract and does the channel handshake for the contract test suite.
	s.SetupContractTestSuite(ctx, encoding)

	// This is the user in our cosmwasm_signer, so we ensure they have funds
	s.FundAddressChainB(ctx, "jkl12g4qwenvpzqeakavx5adqkw203s629tf6k8vdg")

	// Store code of bindings factory
	FactoryCodeId, err := s.ChainB.StoreContract(ctx, s.UserB.KeyName(), "../artifacts/bindings_factory.wasm")
	s.Require().NoError(err)

	// Store code of canine_bindings
	BindingsCodeId, error := s.ChainB.StoreContract(ctx, s.UserB.KeyName(), "../artifacts/canine_bindings.wasm")
	s.Require().NoError(error)

	// codeId is string and needs to be converted to uint64
	BindingsCodeIdAsInt, err := strconv.ParseInt(BindingsCodeId, 10, 64)
	s.Require().NoError(err)

	// Instantiate the factory, giving it the codeId of the canine_bindings contract
	instantiateMsg := factorytypes.InstantiateMsg{BindingsCodeId: int(BindingsCodeIdAsInt)}

	contractAddr, _ := s.ChainB.InstantiateContract(ctx, s.UserB.KeyName(), FactoryCodeId, toString(instantiateMsg), false, "--gas", "500000", "--admin", s.UserB.KeyName())
	// s.Require().NoError(err)

	// NOTE: The above errors only when trying to parse the tx hash, but the instantiate still succeeded
	// We can query for the contract address instead
	// TODO: query for contract address
	fmt.Println(contractAddr)
	logger.LogInfo(contractAddr)

	logger.LogInfo("instantiated factory")

	// NOTE: The contractAddr can't be retrived at this time because of sdk tx parsing error we noted before
	// We can fix that later but for now, we'll just hard code the  consistent factory contract address

	factoryContractAddress := "jkl14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9scsc9nr"

	contractState, stateErr := testsuite.GetState(ctx, s.ChainB, factoryContractAddress)
	s.Require().NoError(stateErr)
	logger.LogInfo(contractState)

	// Fund the factory so it can fund the bindings
	s.FundAddressChainB(ctx, factoryContractAddress)

	s.Run(fmt.Sprintf("TestCreateBindingsSuccess-%s", encoding), func() {

		//****** FOR ALICE ******

		aliceEvmAddress := "alice_Ox1" // Declare a variable holding the string

		// Encode private content bytes to a Base64 string. The CosmWasm contracts will decode this into the Rust
		// equivalent of []byte before sending the message to canine-chain.
		// We handled 'Merkle' of storagetypes.MsgPostFile in the same fashion.

		privateContentBytes := []byte{0x01, 0x02, 0x03, 0x04}
		privateContentBase64 := base64.StdEncoding.EncodeToString(privateContentBytes)

		// WARNING: 'Contents' must be valid json, not just a raw string.
		contentsJson, _ := json.Marshal("nothing") // This becomes `"\"nothing\""`

		createNotificationMsg := allbindingstypes.ExecuteMsg{
			CreateNotification: &allbindingstypes.ExecuteMsg_CreateNotification{
				To:              "nobody",
				Contents:        string(contentsJson),
				PrivateContents: privateContentBase64,
			},
		}

		factoryExecuteMsg := factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &createNotificationMsg,
			},
		}

		res, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		fmt.Println(res)
		expectedErrorMsg := "transaction failed with code 1100: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform create notification: create notification error from message: " +
			"cannot parse address from message: cannot parse the name and tld from given rns: could not extract the tld from the name provided"
		s.Require().EqualError(err, expectedErrorMsg)

		deleteNotificationMsg := allbindingstypes.ExecuteMsg{
			DeleteNotification: &allbindingstypes.ExecuteMsg_DeleteNotification{
				From: "nobody",
				Time: 1000,
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &deleteNotificationMsg,
			},
		}

		res, _ = s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: confirmed via CLI that this works
		fmt.Println(res)

		toBlock := []string{"alice", "bob", "charlie"}

		blockSendersMsg := allbindingstypes.ExecuteMsg{
			BlockSenders: &allbindingstypes.ExecuteMsg_BlockSenders{
				ToBlock: toBlock,
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &blockSendersMsg,
			},
		}

		res, err = s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		fmt.Println(res)
		expectedErrorMsg = "transaction failed with code 1100: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform block senders: block senders error from message: " +
			"cannot parse address alice from message: cannot parse the name and tld from given rns: could not extract the tld from the name provided"
		s.Require().EqualError(err, expectedErrorMsg)
	},
	)
	time.Sleep(time.Duration(10) * time.Hour)
}

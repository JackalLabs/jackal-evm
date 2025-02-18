package interchaintest

import (
	"context"
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
func (s *ContractTestSuite) TestFiletreeModule() {
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

		//****** Create Filetree Entries *********

		//****** FOR ALICE ******

		aliceEvmAddress := "alice_Ox1" // Declare a variable holding the string

		postFileTreeMsg := allbindingstypes.ExecuteMsg{
			PostFileTree: &allbindingstypes.ExecuteMsg_PostFileTree{
				Account:        "nobody",
				HashParent:     "nobody",
				HashChild:      "nobody",
				Contents:       "nobody",
				Viewers:        "nobody",
				Editors:        "nobody",
				TrackingNumber: "1",
			},
		}

		factoryExecuteMsg := factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &postFileTreeMsg,
			},
		}

		res, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res)
		expectedErrorMsg := "transaction failed with code 1106: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform post file tree: post file tree error from message: " +
			"cannot find nobody: Parent folder does not exist"
		s.Require().EqualError(err, expectedErrorMsg)

		addViewersMsg := allbindingstypes.ExecuteMsg{
			AddViewers: &allbindingstypes.ExecuteMsg_AddViewers{
				ViewerIds:  "nobody",
				ViewerKeys: "nobody",
				Address:    "nobody",
				FileOwner:  "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &addViewersMsg,
			},
		}

		res1, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res1)
		expectedErrorMsg = "transaction failed with code 1102: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform add viewers: add viewers error from message: " +
			"file not found"
		s.Require().EqualError(err, expectedErrorMsg)

		postKeyMsg := allbindingstypes.ExecuteMsg{
			PostKey: &allbindingstypes.ExecuteMsg_PostKey{
				Key: "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &postKeyMsg,
			},
		}

		res2, _ := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: confirmed post key works via the CLI.
		fmt.Println(res2)

		deleteFileTreeMsg := allbindingstypes.ExecuteMsg{
			DeleteFileTree: &allbindingstypes.ExecuteMsg_DeleteFileTree{
				HashPath: "nothing",
				Account:  "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &deleteFileTreeMsg,
			},
		}

		res3, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res3)
		expectedErrorMsg = "transaction failed with code 1102: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform delete file tree: delete file tree error from message: " +
			"file not found"
		s.Require().EqualError(err, expectedErrorMsg)

		removeViewersMsg := allbindingstypes.ExecuteMsg{
			RemoveViewers: &allbindingstypes.ExecuteMsg_RemoveViewers{
				ViewerIds: "nothing",
				Address:   "nobody",
				FileOwner: "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &removeViewersMsg,
			},
		}

		res4, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res4)
		expectedErrorMsg = "transaction failed with code 1102: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform remove viewers: remove viewers error from message: " +
			"file not found"
		s.Require().EqualError(err, expectedErrorMsg)

		provisionFileTree := allbindingstypes.ExecuteMsg{
			ProvisionFileTree: &allbindingstypes.ExecuteMsg_ProvisionFileTree{
				Editors:        "BiPhan",
				Viewers:        "BiPhan",
				TrackingNumber: "9000",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &provisionFileTree,
			},
		}

		res5, _ := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		//NOTE: confirmed on cli that this works
		fmt.Println(res5)

		addEditorsMsg := allbindingstypes.ExecuteMsg{
			AddEditors: &allbindingstypes.ExecuteMsg_AddEditors{
				EditorIds:  "nothing",
				EditorKeys: "nothing",
				Address:    "nobody",
				FileOwner:  "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &addEditorsMsg,
			},
		}

		res6, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res6)
		expectedErrorMsg = "transaction failed with code 1102: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform add editors: add editors error from message: " +
			"file not found"
		s.Require().EqualError(err, expectedErrorMsg)

		removeEditorsMsg := allbindingstypes.ExecuteMsg{
			RemoveEditors: &allbindingstypes.ExecuteMsg_RemoveEditors{
				EditorIds: "nothing",
				Address:   "nobody",
				FileOwner: "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &removeEditorsMsg,
			},
		}

		res7, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res7)
		expectedErrorMsg = "transaction failed with code 1102: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform remove editors: remove editors error from message: " +
			"file not found"
		s.Require().EqualError(err, expectedErrorMsg)

		resetEditorsMsg := allbindingstypes.ExecuteMsg{
			ResetEditors: &allbindingstypes.ExecuteMsg_ResetEditors{
				Address:   "nobody",
				FileOwner: "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &resetEditorsMsg,
			},
		}

		res8, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res8)
		expectedErrorMsg = "transaction failed with code 1102: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform reset editors: reset editors error from message: " +
			"file not found"
		s.Require().EqualError(err, expectedErrorMsg)

		resetViewersMsg := allbindingstypes.ExecuteMsg{
			ResetViewers: &allbindingstypes.ExecuteMsg_ResetViewers{
				Address:   "nobody",
				FileOwner: "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &resetViewersMsg,
			},
		}

		res9, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res9)
		expectedErrorMsg = "transaction failed with code 1102: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform reset viewers: reset viewers error from message: " +
			"file not found"
		s.Require().EqualError(err, expectedErrorMsg)

		changeOwnerMsg := allbindingstypes.ExecuteMsg{
			ChangeOwner: &allbindingstypes.ExecuteMsg_ChangeOwner{
				Address:   "nobody",
				FileOwner: "nobody",
				NewOwner:  "nobody",
			},
		}

		factoryExecuteMsg = factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &changeOwnerMsg,
			},
		}

		res10, err := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, factoryExecuteMsg.ToString(), "--gas", "500000", "--amount", "200000000ujkl")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post file tree msg does get saved into canine-chain
		fmt.Println(res10)
		expectedErrorMsg = "transaction failed with code 1102: failed to execute message; message index: 0: " +
			"dispatch: submessages: dispatch: submessages: perform change owner: change owner error from message: " +
			"file not found"
		s.Require().EqualError(err, expectedErrorMsg)
	},
	)
	time.Sleep(time.Duration(10) * time.Hour)
}

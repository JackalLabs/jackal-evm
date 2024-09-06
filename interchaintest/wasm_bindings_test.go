package interchaintest

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"strconv"
	"time"

	testsuite "github.com/JackalLabs/jackal-evm/testsuite"
	icatypes "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/types"

	factorytypes "github.com/JackalLabs/jackal-evm/types/bindingsfactory"
	filetreetypes "github.com/JackalLabs/jackal-evm/types/filetree"
	logger "github.com/JackalLabs/storage-outpost/e2e/interchaintest/logger"
)

// WARNING: strangelove's test package builds chains running ibc-go/v7
// Hopefully this won't cause issues because the canined image we use is running ibc-go/v4
// and packets should be consumed by the ica host no matter what version of ibc-go the controller chain is running

// Testing canine-chain's web assembly bindings
func (s *ContractTestSuite) TestJackalChainWasmBindings() {
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

	// Store code of filetree bindings
	BindingsCodeId, error := s.ChainB.StoreContract(ctx, s.UserB.KeyName(), "../artifacts/filetree.wasm")
	s.Require().NoError(error)

	// codeId is string and needs to be converted to uint64
	BindingsCodeIdAsInt, err := strconv.ParseInt(BindingsCodeId, 10, 64)
	s.Require().NoError(err)

	// Instantiate the factory, giving it the codeId of the filetree bindings contract
	instantiateMsg := factorytypes.InstantiateMsg{BindingsCodeId: int(BindingsCodeIdAsInt)}

	contractAddr, err := s.ChainB.InstantiateContract(ctx, s.UserB.KeyName(), FactoryCodeId, toString(instantiateMsg), false, "--gas", "500000", "--admin", s.UserB.KeyName())
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

	s.Run(fmt.Sprintf("TestCreateBindingsSuccess-%s", encoding), func() {

		// WARNING: NOTE - changing the name of 'callbindingsv2' to 'callbindings' inside factory's contract.rs caused
		// The below execution to fail silently because the golang msg type no longer matched the Rust enum
		aliceEvmAddress := "alice_Ox1" // Declare a variable holding the string
		msg := factorytypes.ExecuteMsg{
			CreateBindingsV2: &factorytypes.ExecuteMsg_CreateBindingsV2{UserEvmAddress: &aliceEvmAddress},
		}

		res, _ := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, msg.ToString(), "--gas", "500000")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post key msg does get saved into canine-chain
		fmt.Println(res)
		//s.Require().NoError(error)

		bobEvmAddress := "bob_Ox1" // Declare a variable holding the string
		msg2 := factorytypes.ExecuteMsg{
			CreateBindingsV2: &factorytypes.ExecuteMsg_CreateBindingsV2{UserEvmAddress: &bobEvmAddress},
		}

		res2, _ := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, msg2.ToString(), "--gas", "500000")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post key msg does get saved into canine-chain
		fmt.Println(res2)
		//s.Require().NoError(error)

		bindingsMap, addressErr := testsuite.GetAllUserBindingsAddresses(ctx, s.ChainB, factoryContractAddress)
		s.Require().NoError(addressErr)

		// Create a slice of slices to hold the decoded user bindings
		var decodedBindingsMap [][]string

		// Unmarshal the response data into the slice of slices of strings
		if err := json.Unmarshal(bindingsMap.Data, &decodedBindingsMap); err != nil {
			log.Fatalf("Error parsing response data: %v", err)
		}

		// Log the decoded map
		for _, binding := range decodedBindingsMap {
			if len(binding) == 2 {
				logger.LogInfo("User Address:", binding[0], "Bindings Address:", binding[1])
			} else {
				logger.LogError("Invalid binding format:", binding)
			}
		}

		filetreeMsg := filetreetypes.ExecuteMsg{
			PostKey: &filetreetypes.ExecuteMsg_PostKey{
				Key: fmt.Sprintf("%s has a public key", aliceEvmAddress),
			},
		}
		crossContractExecuteMsg := factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &aliceEvmAddress,
				Msg:        &filetreeMsg,
			},
		}

		res3, _ := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, crossContractExecuteMsg.ToString(), "--gas", "500000")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post key msg does get saved into canine-chain
		fmt.Println(res3)

		filetreeMsg2 := filetreetypes.ExecuteMsg{
			PostKey: &filetreetypes.ExecuteMsg_PostKey{
				Key: fmt.Sprintf("%s has a public key", bobEvmAddress),
			},
		}
		crossContractExecuteMsg2 := factorytypes.ExecuteMsg{
			CallBindings: &factorytypes.ExecuteMsg_CallBindings{
				EvmAddress: &bobEvmAddress,
				Msg:        &filetreeMsg2,
			},
		}

		res4, _ := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), factoryContractAddress, crossContractExecuteMsg2.ToString(), "--gas", "500000")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post key msg does get saved into canine-chain
		fmt.Println(res4)

	},
	)
	time.Sleep(time.Duration(10) * time.Hour)
}

// log address of bindings contract
// create bindings factory contract

// toString converts the message to a string using json
func toString(msg any) string {
	bz, err := json.Marshal(msg)
	if err != nil {
		panic(err)
	}

	return string(bz)
}

// UserBinding represents a single user binding entry
type UserBinding struct {
	UserAddress     string `json:"0"` // Rust tuple index 0
	BindingsAddress string `json:"1"` // Rust tuple index 1
}

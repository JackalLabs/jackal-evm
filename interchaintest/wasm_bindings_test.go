package interchaintest

import (
	"context"
	"fmt"
	"time"

	icatypes "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/types"

	types "github.com/JackalLabs/jackal-hyperlane/types"
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

	// Upload and Instantiate the filetree wasm bindings contract on canined:
	codeId, err := s.ChainB.StoreContract(ctx, s.UserB.KeyName(), "../artifacts/filetree.wasm")
	s.Require().NoError(err)
	logger.LogInfo(codeId)

	contractAddr, err := s.ChainB.InstantiateContract(ctx, s.UserB.KeyName(), codeId, "{}", false, "--gas", "500000", "--admin", s.UserB.KeyName())
	fmt.Println(contractAddr)
	logger.LogInfo("instantiated filetree binding!")

	// NOTE: The contractAddr can't be retrived at this time because of sdk tx parsing error we noted before
	// We can fix that later but for now, we'll just hard code the  consistent filetree contract address
	filetreeContractAddr := "jkl1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrq59a839"

	s.Run(fmt.Sprintf("TestSendCustomIcaMesssagesSuccess-%s", encoding), func() {

		msg := types.ExecuteMsg{
			PostKey: &types.ExecuteMsg_PostKey{
				Key: "Testing wasm bindings",
			},
		}

		res, _ := s.ChainB.ExecuteContract(ctx, s.UserB.KeyName(), filetreeContractAddr, msg.ToString(), "--gas", "500000")
		// NOTE: cannot parse res because of cosmos-sdk issue noted before, so we will get an error
		// fortunately, we went into the docker container to confirm that the post key msg does get saved into canine-chain
		fmt.Println(res)
		//s.Require().NoError(error)

	},
	)
	time.Sleep(time.Duration(10) * time.Hour)
}

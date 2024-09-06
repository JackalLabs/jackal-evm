package testsuite

import (
	"context"
	"encoding/json"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/strangelove-ventures/interchaintest/v7/chain/cosmos"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func GetBindingsAddressFromFactoryMap(ctx context.Context, chain *cosmos.CosmosChain, factoryContractAddress string, evmUserAddress string) (*wasmtypes.QuerySmartContractStateResponse, error) {
	grpcConn, err := grpc.Dial(
		chain.GetHostGRPCAddress(),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	if err != nil {
		return nil, err
	}
	defer grpcConn.Close()
	queryClient := wasmtypes.NewQueryClient(grpcConn)

	// TODO: replace with query msg type in types/outpostfactory/msg.go
	queryData := map[string]interface{}{
		"get_user_bindings_address": map[string]string{
			"user_address": evmUserAddress,
		},
	}

	queryDataBytes, err := json.Marshal(queryData)
	if err != nil {
		return nil, err
	}

	params := &wasmtypes.QuerySmartContractStateRequest{
		Address:   factoryContractAddress,
		QueryData: queryDataBytes,
	}
	return queryClient.SmartContractState(ctx, params)
}

// GetAllUserBindingsAddresses queries the contract for all user bindings addresses
func GetAllUserBindingsAddresses(ctx context.Context, chain *cosmos.CosmosChain, factoryContractAddress string) (*wasmtypes.QuerySmartContractStateResponse, error) {
	grpcConn, err := grpc.Dial(
		chain.GetHostGRPCAddress(),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	if err != nil {
		return nil, err
	}
	defer grpcConn.Close()

	queryClient := wasmtypes.NewQueryClient(grpcConn)

	// Create the query message for fetching all user bindings addresses
	queryData := map[string]interface{}{
		"get_all_user_bindings_addresses": struct{}{},
	}

	queryDataBytes, err := json.Marshal(queryData)
	if err != nil {
		return nil, err
	}

	params := &wasmtypes.QuerySmartContractStateRequest{
		Address:   factoryContractAddress,
		QueryData: queryDataBytes,
	}

	return queryClient.SmartContractState(ctx, params)
}

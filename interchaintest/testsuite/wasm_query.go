package testsuite

import (
	"context"
	"encoding/json"
	"fmt"

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

// GetState queries and returns 'ContractState' object. Can be used for either the factory or the bindings contract(s)
func GetState(ctx context.Context, chain *cosmos.CosmosChain, contractAddress string) (*wasmtypes.QuerySmartContractStateResponse, error) {
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
		"get_contract_state": struct{}{},
	}

	queryDataBytes, err := json.Marshal(queryData)
	if err != nil {
		return nil, err
	}

	params := &wasmtypes.QuerySmartContractStateRequest{
		Address:   contractAddress,
		QueryData: queryDataBytes,
	}

	return queryClient.SmartContractState(ctx, params)
}

// GetWhiteList queries and returns entire Whitelist
func GetWhiteList(ctx context.Context, chain *cosmos.CosmosChain, factoryContractAddress string) (*wasmtypes.QuerySmartContractStateResponse, error) {
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
		"get_white_list": struct{}{},
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

// GetAllBroadcastedMsgs queries the contract for all broadcasted messages.
func GetAllBroadcastedMsgs(ctx context.Context, chain *cosmos.CosmosChain, factoryContractAddress string) (*wasmtypes.QuerySmartContractStateResponse, error) {
	// Establish gRPC connection
	grpcConn, err := grpc.Dial(
		chain.GetHostGRPCAddress(),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	if err != nil {
		return nil, err
	}
	defer grpcConn.Close()

	queryClient := wasmtypes.NewQueryClient(grpcConn)

	// Create the query message for fetching all broadcasted messages
	queryData := map[string]interface{}{
		"get_all_broadcasted_msgs": struct{}{},
	}

	queryDataBytes, err := json.Marshal(queryData)
	if err != nil {
		return nil, err
	}

	params := &wasmtypes.QuerySmartContractStateRequest{
		Address:   factoryContractAddress,
		QueryData: queryDataBytes,
	}

	// Execute the query
	return queryClient.SmartContractState(ctx, params)
}

// BroadcastedMsg represents a single entry in the query response
type BroadcastedMsg struct {
	UserAddress string
	MsgHash     string
	Status      bool
}

// DecodeGetAllBroadcastedMsgsResponse decodes the query response correctly
func DecodeGetAllBroadcastedMsgsResponse(resp *wasmtypes.QuerySmartContractStateResponse) ([]BroadcastedMsg, error) {
	// Step 1: Decode JSON response into a slice of slices
	var rawMessages [][]interface{}
	err := json.Unmarshal(resp.Data, &rawMessages)
	if err != nil {
		return nil, err
	}

	// Step 2: Convert into BroadcastedMsg struct
	var messages []BroadcastedMsg
	for _, raw := range rawMessages {
		if len(raw) != 3 {
			return nil, fmt.Errorf("unexpected tuple format: %+v", raw)
		}

		// Type assertion for each field
		user, ok1 := raw[0].(string)
		msgHash, ok2 := raw[1].(string)
		status, ok3 := raw[2].(bool)

		if !ok1 || !ok2 || !ok3 {
			return nil, fmt.Errorf("type assertion failed for: %+v", raw)
		}

		messages = append(messages, BroadcastedMsg{
			UserAddress: user,
			MsgHash:     msgHash,
			Status:      status,
		})
	}

	return messages, nil
}

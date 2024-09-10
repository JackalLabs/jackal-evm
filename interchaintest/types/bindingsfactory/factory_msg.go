package bindingsfactory

import (
	"encoding/json"

	filetreetypes "github.com/JackalLabs/jackal-evm/types/filetree"
	storagetypes "github.com/JackalLabs/jackal-evm/types/storage"
)

type InstantiateMsg struct {
	BindingsCodeId int `json:"bindings_code_id"`
}

// not sure if 'create_bindings_v2' is correct
type ExecuteMsg struct {
	CreateBindingsV2    *ExecuteMsg_CreateBindingsV2    `json:"create_bindings_v2,omitempty"`
	CallBindings        *ExecuteMsg_CallBindings        `json:"call_bindings,omitempty"`
	CallStorageBindings *ExecuteMsg_CallStorageBindings `json:"call_storage_bindings,omitempty"`
}

type ExecuteMsg_CreateBindingsV2 struct {
	UserEvmAddress *string `json:"user_evm_address,omitempty"`
}

type ExecuteMsg_CallBindings struct {
	EvmAddress *string                   `json:"evm_address,omitempty"`
	Msg        *filetreetypes.ExecuteMsg `json:"msg,omitempty"`
}

type ExecuteMsg_CallStorageBindings struct {
	EvmAddress *string                  `json:"evm_address,omitempty"`
	Msg        *storagetypes.ExecuteMsg `json:"msg,omitempty"`
}

// ToString returns a string representation of the message
func (m *ExecuteMsg) ToString() string {
	return toString(m)
}

func toString(v any) string {
	jsonBz, err := json.Marshal(v)
	if err != nil {
		panic(err)
	}

	return string(jsonBz)
}

package bindingsfactory

import (
	"encoding/json"

	allbindingtypes "github.com/JackalLabs/jackal-evm/types/bindings"
)

type InstantiateMsg struct {
	BindingsCodeId int `json:"bindings_code_id"`
}

// not sure if 'create_bindings_v2' is correct
type ExecuteMsg struct {
	CreateBindings *ExecuteMsg_CreateBindings `json:"create_bindings,omitempty"`
	FundBindings   *ExecuteMsg_FundBindings   `json:"fund_bindings,omitempty"`
	CallBindings   *ExecuteMsg_CallBindings   `json:"call_bindings,omitempty"`
	AddToWhiteList *ExecuteMsg_AddToWhiteList `json:"add_to_white_list,omitempty"`
	InitAccount    *ExecuteMsg_InitAccount    `json:"init_account,omitempty"`
}

type ExecuteMsg_AddToWhiteList struct {
	JKLAddress *string `json:"jkl_address,omitempty"`
}

type ExecuteMsg_CreateBindings struct {
	UserEvmAddress *string `json:"user_evm_address,omitempty"`
}

// TODO: rename this to 'CallFiletreeBindings'
type ExecuteMsg_CallBindings struct {
	EvmAddress *string                     `json:"evm_address,omitempty"`
	Msg        *allbindingtypes.ExecuteMsg `json:"msg,omitempty"`
}

type ExecuteMsg_InitAccount struct {
	EvmAddress *string `json:"evm_address,omitempty"`
}

type ExecuteMsg_FundBindings struct {
	EvmAddress *string `json:"evm_address,omitempty"`
	Amount     *int64  `json:"amount,omitempty"`
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

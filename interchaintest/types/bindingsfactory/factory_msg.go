package bindingsfactory

import "encoding/json"

type InstantiateMsg struct {
	BindingsCodeId int `json:"bindings_code_id"`
}

// not sure if 'create_bindings_v2' is correct
type ExecuteMsg struct {
	CreateBindingsV2 *ExecuteMsg_CreateBindingsV2 `json:"create_bindings_v2,omitempty"`
}

type ExecuteMsg_CreateBindingsV2 struct {
	// TODO: add args
	// Salt                   *string                `json:"salt,omitempty"`
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

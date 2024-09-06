package filetree

// helper functions to create json msgs for CosmWasm instantiate, execute, and migrate
import (
	"encoding/json"
)

// WARNING: I think this needs to be in a folder called 'filetree_types' because mailbox_msg.go will also need an 'ExecuteMsg' enum
// ExecuteMsg is the message to execute the filetree contract.
type ExecuteMsg struct {
	PostKey *ExecuteMsg_PostKey `json:"post_key,omitempty"`
}

// `CreateTransferChannel` is opening a transfer channel
// for development purposees only. Not using ChannelOpenInitOptions
type ExecuteMsg_PostKey struct {
	Key string `json:"key"`
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

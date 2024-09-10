package filetree

// helper functions to create json msgs for CosmWasm instantiate, execute, and migrate
import (
	"encoding/json"
)

// WARNING: I think this needs to be in a folder called 'filetree_types' because mailbox_msg.go will also need an 'ExecuteMsg' enum
// ExecuteMsg is the message to execute the filetree contract.
type ExecuteMsg struct {
	PostKey  *ExecuteMsg_PostKey  `json:"post_key,omitempty"`
	PostFile *ExecuteMsg_PostFile `json:"post_file,omitempty"`
}

type ExecuteMsg_PostKey struct {
	Key string `json:"key"`
}

type ExecuteMsg_PostFile struct {
	Merkle        []byte `json:"merkle"`
	FileSize      int64  `json:"file_size"`
	ProofInterval int64  `json:"proof_interval"`
	ProofType     int64  `json:"proof_type"`
	MaxProofs     int64  `json:"max_proofs"`
	Expires       int64  `json:"expires"`
	Note          string `json:"note"`
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

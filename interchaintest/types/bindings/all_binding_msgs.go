package bindings

// helper functions to create json msgs for CosmWasm instantiate, execute, and migrate
import (
	"encoding/json"
)

type ExecuteMsg struct {
	PostKey    *ExecuteMsg_PostKey    `json:"post_key,omitempty"`
	PostFile   *ExecuteMsg_PostFile   `json:"post_file,omitempty"`
	BuyStorage *ExecuteMsg_BuyStorage `json:"buy_storage,omitempty"`
}

type ExecuteMsg_PostKey struct {
	Key string `json:"key"`
}

type ExecuteMsg_PostFile struct {
	Merkle        string `json:"merkle"`
	FileSize      int64  `json:"file_size"`
	ProofInterval int64  `json:"proof_interval"`
	ProofType     int64  `json:"proof_type"`
	MaxProofs     int64  `json:"max_proofs"`
	Expires       int64  `json:"expires"`
	Note          string `json:"note"`
}

type ExecuteMsg_BuyStorage struct {
	ForAddress   string `json:"for_address"`
	DurationDays int64  `json:"duration_days"`
	Bytes        int64  `json:"bytes"`
	PaymentDenom string `json:"payment_denom"`
	Referral     string `json:"referral"`
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

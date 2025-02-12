package bindings

// helper functions to create json msgs for CosmWasm instantiate, execute, and migrate
import (
	"encoding/json"
)

type ExecuteMsg struct {
	// STORAGE
	PostFile          *ExecuteMsg_PostFile          `json:"post_file,omitempty"`
	DeleteFile        *ExecuteMsg_DeleteFile        `json:"delete_file,omitempty"`
	BuyStorage        *ExecuteMsg_BuyStorage        `json:"buy_storage,omitempty"`
	RequestReportForm *ExecuteMsg_RequestReportForm `json:"request_report_form,omitempty"`
	// FILETREE
	PostFileTree      *ExecuteMsg_PostFileTree      `json:"post_file_tree,omitempty"`
	AddViewers        *ExecuteMsg_AddViewers        `json:"add_viewers,omitempty"`
	PostKey           *ExecuteMsg_PostKey           `json:"post_key,omitempty"`
	DeleteFileTree    *ExecuteMsg_DeleteFileTree    `json:"delete_file_tree,omitempty"`
	RemoveViewers     *ExecuteMsg_RemoveViewers     `json:"remove_viewers,omitempty"`
	ProvisionFileTree *ExecuteMsg_ProvisionFileTree `json:"provision_file_tree,omitempty"`
	AddEditors        *ExecuteMsg_AddEditors        `json:"add_editors,omitempty"`
	RemoveEditors     *ExecuteMsg_RemoveEditors     `json:"remove_editors,omitempty"`
	ResetEditors      *ExecuteMsg_ResetEditors      `json:"reset_editors,omitempty"`
	ResetViewers      *ExecuteMsg_ResetViewers      `json:"reset_viewers,omitempty"`
	ChangeOwner       *ExecuteMsg_ChangeOwner       `json:"change_owner,omitempty"`
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

type ExecuteMsg_DeleteFile struct {
	Merkle string `json:"merkle"`
	Start  int64  `json:"start"`
}

type ExecuteMsg_BuyStorage struct {
	ForAddress   string `json:"for_address"`
	DurationDays int64  `json:"duration_days"`
	Bytes        int64  `json:"bytes"`
	PaymentDenom string `json:"payment_denom"`
	Referral     string `json:"referral"`
}

type ExecuteMsg_RequestReportForm struct {
	Prover string `json:"prover"`
	Merkle string `json:"merkle"`
	Owner  string `json:"owner"`
	Start  int64  `json:"start"`
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

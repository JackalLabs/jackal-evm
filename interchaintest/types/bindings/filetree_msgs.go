package bindings

type ExecuteMsg_PostFileTree struct {
	Account        string `json:"account"`
	HashParent     string `json:"hash_parent"`
	HashChild      string `json:"hash_child"`
	Contents       string `json:"contents"`
	Viewers        string `json:"viewers"`
	Editors        string `json:"editors"`
	TrackingNumber string `json:"tracking_number"`
}

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

type ExecuteMsg_AddViewers struct {
	ViewerIds  string `json:"viewer_ids"`
	ViewerKeys string `json:"viewer_keys"`
	Address    string `json:"address"`
	FileOwner  string `json:"file_owner"`
}

type ExecuteMsg_PostKey struct {
	Key string `json:"key"`
}

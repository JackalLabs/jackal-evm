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

type ExecuteMsg_DeleteFileTree struct {
	HashPath string `json:"hash_path"`
	Account  string `json:"account"`
}

type ExecuteMsg_RemoveViewers struct {
	ViewerIds string `json:"viewer_ids"`
	Address   string `json:"address"`
	FileOwner string `json:"file_owner"`
}

type ExecuteMsg_ProvisionFileTree struct {
	Editors        string `json:"editors"`
	Viewers        string `json:"viewers"`
	TrackingNumber string `json:"tracking_number"`
}

type ExecuteMsg_AddEditors struct {
	EditorIds  string `json:"editor_ids"`
	EditorKeys string `json:"editor_keys"`
	Address    string `json:"address"`
	FileOwner  string `json:"file_owner"`
}

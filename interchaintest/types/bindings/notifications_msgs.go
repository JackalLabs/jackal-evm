package bindings

type ExecuteMsg_CreateNotification struct {
	To              string `json:"to"`
	Contents        string `json:"contents"`
	PrivateContents string `json:"private_contents"`
}

type ExecuteMsg_DeleteNotification struct {
	From string `json:"from"`
	Time int64  `json:"time"`
}

type ExecuteMsg_BlockSenders struct {
	ToBlock []string `json:"to_block"`
}

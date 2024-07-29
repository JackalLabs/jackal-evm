package types

// helper functions to create json msgs for CosmWasm instantiate, execute, and migrate
import (
	"encoding/json"
)

// NOTE: what the mailbox expects
// pub struct InstantiateMsg {
//     pub hrp: String,
//     pub owner: String,
//     pub domain: u32,
// }

func NewMailboxInstantiateMsg(
	hrp string, //more args
	owner string,
	domain uint32,
) string {
	type InstantiateMsg struct {
		// hrp
		Hrp string `json:"hrp,omitempty"`
		// owner
		Owner string `json:"owner,omitempty"`
		// domain
		Domain uint32 `json:"domain,omitempty"`
	}

	instantiateMsg := InstantiateMsg{
		Hrp:    hrp,
		Owner:  owner,
		Domain: domain,
	}

	jsonBytes, err := json.Marshal(instantiateMsg)
	if err != nil {
		panic(err)
	}

	return string(jsonBytes)
}

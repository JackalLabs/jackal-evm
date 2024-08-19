use cosmrs::proto::prost::Message;
use tendermint_rpc::{Client, HttpClient};
use tendermint_rpc::endpoint::abci_query::AbciQuery;
use cosmrs::proto::cosmos::auth::v1beta1::BaseAccount;
use anyhow::Result;

pub(crate) async fn get_account_sequence_number(address: String, rpc_url: String) -> Result<u64> {
    // Initialize the Tendermint RPC client
    let client = HttpClient::new(rpc_url.as_str())?;

    println!("rpc url: {}", rpc_url );
    println!("address: {}", address );

    // Prepare the ABCI query to fetch the account information
    // need a slash in the front?
    let query_path = format!("cosmos.auth.v1beta1.Query/Account/{}", address);
    // NOTE: keeps returning sequence of '1'
    println!("query path: {}", query_path );


    // Send the query to the ABCI endpoint
    let response: AbciQuery = client
        .abci_query(Some(query_path), vec![], None, false)
        .await?;

    // Extract and decode the account information from the response
    let account_data = response.value;

    // Deserialize the BaseAccount protobuf message
    let base_account: BaseAccount = BaseAccount::decode(account_data.as_slice())?;
    println!("base account: {:?}", base_account);

    // Return the sequence number
    Ok(base_account.sequence)
}

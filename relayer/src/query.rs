use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use prost_types::Any;
use tonic::transport::Channel;
use prost::Message; // Ensure prost::Message trait is in scope
use anyhow::Result;

pub(crate) async fn get_account_sequence_number(address: String, grpc_url: String) -> Result<u64> {
    println!("grpc url is: {}", grpc_url.clone());
    let mut client = QueryClient::connect(grpc_url.clone()).await?;
    let request = QueryAccountRequest { address };
    let response = client.account(request).await?;

    let any = response.into_inner().account.unwrap();



    // Decode the BaseAccount manually
    if any.type_url == "/cosmos.auth.v1beta1.BaseAccount" {
        let base_account: BaseAccount = BaseAccount::decode(any.value.as_slice())?;
        Ok(base_account.sequence)
    } else {
        Err(anyhow::anyhow!("Unexpected account type"))
    }
}

use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use tonic::transport::Channel;
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;

async fn get_account_sequence_number(address: String, grpc_url: String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut client = QueryClient::connect(grpc_url).await?;
    let request = QueryAccountRequest { address };
    let response = client.account(request).await?;
    
        // Manually decode the Any type into a BaseAccount
        let any = response.into_inner().account.unwrap();
        let base_account = BaseAccount::decode(any.value.as_slice())?;
    Ok(base_account.sequence)
}

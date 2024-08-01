use mini_redis::{client, Result};

// Multi threaded by default. If the cosmwasm signer uses the multi threaded runtime, it will likely run into sequence mismatch errors
// Will need to use this to ensure everything runs on one thread: #[tokio::main(flavor = "current_thread")]

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with the value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}



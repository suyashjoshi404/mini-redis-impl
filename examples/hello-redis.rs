use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    //creating a client and opening a connection
    let mut client = client::connect("127.0.0.1:6379").await?;

    //setting the key and the value
    client.set("hello", "world".into()).await?;

    //obtaining the key
    let result = client.get("hello").await?;

    println!("Got the value from the server; result = {:?}", result);

    Ok(())
}

use reqwest::Client;
use srcapi_plus::api::client::SrcClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = SrcClient::new(Client::new());

    let id = String::from("1xyy2qyx");

    let user = client.get_user(id).await?;

    println!("{:#?}", user);

    Ok(())
}

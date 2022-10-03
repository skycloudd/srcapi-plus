use srcapi_plus::client::SrcClient;
use std::error::Error;
use std::string::String;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = SrcClient::new();

    let id = String::from("j0ng00m8");

    let users = client.get_user_pbs(id, None, None, None).await?;

    for user in users {
        println!(
            "[{}] {}",
            user.location.country.code.to_uppercase(),
            user.names.international,
        );
    }

    Ok(())
}

use srcapi_plus::client::SrcClient;
use std::error::Error;
use std::string::String;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = SrcClient::new();

    let name = String::from("kyraa");

    let users = client
        .get_users(None, Some(name), None, None, None, None, None, None)
        .await?;

    for user in users {
        println!(
            "[{}] {}",
            user.location.country.code.to_uppercase(),
            user.names.international,
        );
    }

    let id = String::from("j0ng00m8");

    let user = client.get_user(id).await?;

    println!(
        "[{}] {}",
        user.location.country.code.to_uppercase(),
        user.names.international,
    );

    let abbr = String::from("mc");

    let games = client
        .get_games(
            None,
            Some(abbr),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;

    for game in games {
        println!("{} - {}", game.id, game.names.international);

        for moderator in game.moderators {
            println!("\t{} - {}", moderator.0, moderator.1);
        }
    }

    Ok(())
}

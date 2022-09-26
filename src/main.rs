use anyhow::Error;
use srcapi_plus::api::{
    query::{query, Query},
    users::UserQuery,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let id = "j0ng00m8".to_string();

    let user: UserQuery = query(Query::User { id }).await?;

    println!("{:#?}", user);

    Ok(())
}

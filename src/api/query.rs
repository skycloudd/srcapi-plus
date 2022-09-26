use super::BASE_URL;
use anyhow::Error;
use reqwest::Url;
use serde::de::DeserializeOwned;

pub async fn query<T>(query: Query) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let mut url = Url::parse(BASE_URL)?;

    match query {
        Query::User { id } => {
            url = url.join("users/")?.join(&id)?;
        }
        Query::Users {
            lookup,
            name,
            twitch,
            hitbox,
            twitter,
            speedrunslive,
        } => {
            url = url.join("users")?;

            if let Some(v) = lookup {
                url.query_pairs_mut().append_pair("lookup", &v);
            } else {
                if let Some(v) = name {
                    url.query_pairs_mut().append_pair("name", &v);
                }
                if let Some(v) = twitch {
                    url.query_pairs_mut().append_pair("twitch", &v);
                }
                if let Some(v) = hitbox {
                    url.query_pairs_mut().append_pair("hitbox", &v);
                }
                if let Some(v) = twitter {
                    url.query_pairs_mut().append_pair("twitter", &v);
                }
                if let Some(v) = speedrunslive {
                    url.query_pairs_mut().append_pair("speedrunslive", &v);
                }
            }
        }
    }

    Ok(serde_json::from_str(
        &reqwest::get(url).await?.text().await?,
    )?)
}

pub enum Query {
    User {
        id: String,
    },
    Users {
        lookup: Option<String>,
        name: Option<String>,
        twitch: Option<String>,
        hitbox: Option<String>,
        twitter: Option<String>,
        speedrunslive: Option<String>,
    },
}

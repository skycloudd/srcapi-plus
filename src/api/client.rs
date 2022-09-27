use std::error::Error;

use reqwest::Client;

use super::{
    query::{query, QueryData, QueryType},
    users::{User, User_, Users_},
};

pub struct SrcClient {
    client: Client,
}

impl SrcClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_user(&self, id: String) -> Result<User, Box<dyn Error>> {
        let q = QueryData::new(QueryType::User { id });

        let user_: User_ = query(&self.client, &q).await?;

        Ok(user_.data)
    }

    pub async fn get_users(
        &self,
        lookup: Option<String>,
        name: Option<String>,
        twitch: Option<String>,
        hitbox: Option<String>,
        twitter: Option<String>,
        speedrunslive: Option<String>,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let mut q = QueryData::new(QueryType::Users {});

        if let Some(lookup) = lookup {
            q.param("lookup", lookup)?;
        }
        if let Some(name) = name {
            q.param("name", name)?;
        }
        if let Some(twitch) = twitch {
            q.param("twitch", twitch)?;
        }
        if let Some(hitbox) = hitbox {
            q.param("hitbox", hitbox)?;
        }
        if let Some(twitter) = twitter {
            q.param("twitter", twitter)?;
        }
        if let Some(speedrunslive) = speedrunslive {
            q.param("speedrunslive", speedrunslive)?;
        }

        let users_: Users_ = query(&self.client, &q).await?;

        Ok(users_.data)
    }
}

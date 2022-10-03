use crate::{
    query::{query, OrderBy, OrderDirection, Parameter, QueryData, QueryType},
    users::User,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct SrcClient {
    pub client: Client,
}

impl SrcClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_user(&self, id: String) -> Result<User, Box<dyn Error>> {
        let q = QueryData::new(QueryType::User { id });

        let user_: UserData = query(&self, &q).await?;

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
        orderby: Option<OrderBy>,
        direction: Option<OrderDirection>,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let mut q = QueryData::new(QueryType::Users {});

        if let Some(lookup) = lookup {
            q.params.push(Parameter::Lookup(lookup.to_string()));
        }
        if let Some(name) = name {
            q.params.push(Parameter::Lookup(name.to_string()));
        }
        if let Some(twitch) = twitch {
            q.params.push(Parameter::Lookup(twitch.to_string()));
        }
        if let Some(hitbox) = hitbox {
            q.params.push(Parameter::Lookup(hitbox.to_string()));
        }
        if let Some(twitter) = twitter {
            q.params.push(Parameter::Lookup(twitter.to_string()));
        }
        if let Some(speedrunslive) = speedrunslive {
            q.params.push(Parameter::Lookup(speedrunslive.to_string()));
        }
        if let Some(orderby) = orderby {
            q.params.push(Parameter::OrderBy(orderby));
        }
        if let Some(direction) = direction {
            q.params.push(Parameter::Direction(direction));
        }

        Ok(query::<UsersData>(&self, &q).await?.data)
    }

    pub async fn get_user_pbs(
        &self,
        id: String,
        top: Option<u32>,
        series: Option<String>,
        game: Option<String>,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let mut q = QueryData::new(QueryType::UserPBs { id });

        if let Some(top) = top {
            q.params.push(Parameter::Top(top))
        }
        if let Some(series) = series {
            q.params.push(Parameter::Series(series.to_string()))
        }
        if let Some(game) = game {
            q.params.push(Parameter::Game(game.to_string()))
        }

        let users_: UsersData = query(&self, &q).await?;

        Ok(users_.data)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UsersData {
    data: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    data: User,
}

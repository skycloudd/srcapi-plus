use crate::{
    games::Game,
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
        top: Option<i32>,
        series: Option<String>,
        game: Option<String>,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let mut q = QueryData::new(QueryType::UserPBs { id });

        if let Some(top) = top {
            q.params.push(Parameter::Top(top))
        }
        if let Some(series) = series {
            q.params.push(Parameter::Series(series))
        }
        if let Some(game) = game {
            q.params.push(Parameter::Game(game))
        }

        let users_: UsersData = query(&self, &q).await?;

        Ok(users_.data)
    }

    pub async fn get_games(
        &self,
        name: Option<String>,
        abbreviation: Option<String>,
        released: Option<i32>,
        gametype: Option<String>,
        platform: Option<String>,
        region: Option<String>,
        genre: Option<String>,
        engine: Option<String>,
        developer: Option<String>,
        publisher: Option<String>,
        moderator: Option<String>,
    ) -> Result<Vec<Game>, Box<dyn Error>> {
        let mut q = QueryData::new(QueryType::Games);

        if let Some(name) = name {
            q.params.push(Parameter::Name(name))
        }
        if let Some(abbreviation) = abbreviation {
            q.params.push(Parameter::Abbreviation(abbreviation))
        }
        if let Some(released) = released {
            q.params.push(Parameter::Released(released))
        }
        if let Some(gametype) = gametype {
            q.params.push(Parameter::Gametype(gametype))
        }
        if let Some(platform) = platform {
            q.params.push(Parameter::Platform(platform))
        }
        if let Some(region) = region {
            q.params.push(Parameter::Region(region))
        }
        if let Some(genre) = genre {
            q.params.push(Parameter::Genre(genre))
        }
        if let Some(engine) = engine {
            q.params.push(Parameter::Engine(engine))
        }
        if let Some(developer) = developer {
            q.params.push(Parameter::Developer(developer))
        }
        if let Some(publisher) = publisher {
            q.params.push(Parameter::Publisher(publisher))
        }
        if let Some(moderator) = moderator {
            q.params.push(Parameter::Moderator(moderator))
        }

        let users_: GamesData = query(&self, &q).await?;

        Ok(users_.data)
    }
}

#[derive(Serialize, Deserialize)]
struct UsersData {
    data: Vec<User>,
}

#[derive(Serialize, Deserialize)]
struct UserData {
    data: User,
}

#[derive(Serialize, Deserialize)]
struct GamesData {
    data: Vec<Game>,
}

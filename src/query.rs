use crate::client::SrcClient;
use crate::BASE_URL;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::error::Error;
use thiserror::Error;

pub async fn query<T>(client: &SrcClient, query: &dyn Query) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let mut url = Url::parse(BASE_URL)?;

    url = url.join(&match query.query_type() {
        QueryType::User { ref id } => format!("users/{}", id),
        QueryType::Users => "users".to_string(),
        QueryType::UserPBs { ref id } => format!("users/{}/personal-bests", id),
    })?;

    for param in query.params() {
        let (name, value) = match param {
            Parameter::Lookup(s) => (String::from("lookup"), s),
            Parameter::Name(s) => (String::from("name"), s),
            Parameter::Twitch(s) => (String::from("twitch"), s),
            Parameter::Hitbox(s) => (String::from("hitbox"), s),
            Parameter::Twitter(s) => (String::from("twitter"), s),
            Parameter::Speedrunslive(s) => (String::from("speedrunslive"), s),
            Parameter::OrderBy(o) => (
                String::from("orderby"),
                match o {
                    OrderBy::NameInt => String::from("name.int"),
                    OrderBy::NameJap => String::from("name.jap"),
                    OrderBy::Signup => String::from("signup"),
                    OrderBy::Role => String::from("role"),
                },
            ),
            Parameter::Direction(o) => (
                String::from("direction"),
                match o {
                    OrderDirection::Asc => String::from("asc"),
                    OrderDirection::Desc => String::from("desc"),
                },
            ),
            Parameter::Top(u) => (String::from("top"), u.to_string()),
            Parameter::Series(s) => (String::from("series"), s),
            Parameter::Game(s) => (String::from("game"), s),
        };

        url.query_pairs_mut().append_pair(&name, &value);
    }

    // validate url

    let count = url.query_pairs().count();

    match query.query_type() {
        QueryType::User { .. } => {
            if count > 0 {
                return Err(QueryError::WrongParamCountNeq {
                    expected: 0,
                    got: count,
                }
                .into());
            }
        }
        QueryType::Users => {
            if count == 0 {
                return Err(QueryError::WrongParamCountGt {
                    expected: 0,
                    got: count,
                }
                .into());
            }
        }
        QueryType::UserPBs { .. } => {}
    }

    Ok(serde_json::from_str(
        &client.client.get(url).send().await?.text().await?,
    )?)
}

pub trait Query {
    fn query_type(&self) -> QueryType;
    fn params(&self) -> Vec<Parameter>;
}

#[derive(Clone)]
pub struct QueryData {
    query_type: QueryType,
    pub params: Vec<Parameter>,
}

impl QueryData {
    pub fn new(query_type: QueryType) -> Self {
        Self {
            query_type,
            params: Vec::new(),
        }
    }
}

impl Query for QueryData {
    fn query_type(&self) -> QueryType {
        self.query_type.clone()
    }

    fn params(&self) -> Vec<Parameter> {
        self.params.clone()
    }
}

#[derive(Clone)]
pub enum QueryType {
    User { id: String },
    Users,
    UserPBs { id: String },
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Parameter {
    Lookup(String),
    Name(String),
    Twitch(String),
    Hitbox(String),
    Twitter(String),
    Speedrunslive(String),
    OrderBy(OrderBy),
    Direction(OrderDirection),
    Top(u32),
    Series(String),
    Game(String),
}

#[derive(Clone)]
pub enum OrderBy {
    NameInt,
    NameJap,
    Signup,
    Role,
}

#[derive(Clone)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[allow(dead_code)]
#[derive(Error, Debug)]
enum QueryError {
    #[error("Invalid query parameter: {name}={value}")]
    InvalidParam { name: String, value: String },

    #[error("Incorrect amount of parameters specified (expected {expected}, got {got})")]
    WrongParamCountNeq { expected: usize, got: usize },

    #[error("Incorrect amount of parameters specified (expected <{expected}, got {got})")]
    WrongParamCountLt { expected: usize, got: usize },

    #[error("Incorrect amount of parameters specified (expected <={expected}, got {got})")]
    WrongParamCountLte { expected: usize, got: usize },

    #[error("Incorrect amount of parameters specified (expected >{expected}, got {got})")]
    WrongParamCountGt { expected: usize, got: usize },

    #[error("Incorrect amount of parameters specified (expected >={expected}, got {got})")]
    WrongParamCountGte { expected: usize, got: usize },

    #[error("Invalid parameter type: {param_type}")]
    InvalidParamType { param_type: String },
}

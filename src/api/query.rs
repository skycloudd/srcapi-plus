use super::BASE_URL;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::error::Error;
use thiserror::Error;

pub(crate) async fn query<T>(client: &Client, query: &dyn Query) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let mut url = Url::parse(BASE_URL)?;

    url = url.join(&match query.query_type() {
        QueryType::User { ref id } => format!("users/{}", id),
        QueryType::Users => "users".to_string(),
    })?;

    for param in &query.params() {
        url.query_pairs_mut()
            .append_pair((&param.0).try_into()?, &param.1);
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
    }

    Ok(serde_json::from_str(
        &client.get(url).send().await?.text().await?,
    )?)
}

pub(crate) trait Query {
    fn query_type(&self) -> QueryType;
    fn params(&self) -> Vec<Parameter>;
}

#[derive(Clone)]
pub(crate) struct QueryData {
    query_type: QueryType,
    params: Vec<Parameter>,
}

impl QueryData {
    pub(crate) fn new(query_type: QueryType) -> Self {
        Self {
            query_type,
            params: Vec::new(),
        }
    }

    pub(crate) fn param(&mut self, name: &str, value: String) -> Result<&mut Self, Box<dyn Error>> {
        self.params.push((name.try_into()?, value));

        Ok(self)
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
pub(crate) enum QueryType {
    User { id: String },
    Users,
}

pub(crate) type Parameter = (ParamType, String);

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) enum ParamType {
    Lookup,
    Name,
    Twitch,
    Hitbox,
    Twitter,
    Speedrunslive,
}

impl TryFrom<&ParamType> for &str {
    type Error = Box<dyn Error>;

    fn try_from(name: &ParamType) -> Result<Self, Self::Error> {
        match name {
            ParamType::Lookup => Ok("lookup"),
            ParamType::Name => Ok("name"),
            ParamType::Twitch => Ok("twitch"),
            ParamType::Hitbox => Ok("hitbox"),
            ParamType::Twitter => Ok("twitter"),
            ParamType::Speedrunslive => Ok("speedrunslive"),
        }
    }
}

impl TryFrom<&str> for ParamType {
    type Error = Box<dyn Error>;

    fn try_from(name: &str) -> Result<Self, <ParamType as TryFrom<&str>>::Error> {
        match name {
            "lookup" => Ok(ParamType::Lookup),
            "name" => Ok(ParamType::Name),
            "twitch" => Ok(ParamType::Twitch),
            "hitbox" => Ok(ParamType::Hitbox),
            "twitter" => Ok(ParamType::Twitter),
            "speedrunslive" => Ok(ParamType::Speedrunslive),
            invalid => Err(QueryError::InvalidParamType {
                param_type: invalid.to_string(),
            }
            .into()),
        }
    }
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

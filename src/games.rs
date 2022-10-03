use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: GameId,
    pub names: Names,
    pub abbreviation: GameAbbreviation,
    pub weblink: Weblink,
    #[deprecated = "released is a legacy value that has been superceded by release_date"]
    pub released: i32,
    #[serde(rename = "release-date")]
    pub release_date: ReleaseDate,
    pub ruleset: Ruleset,
    pub romhack: bool,
    pub gametypes: Vec<GametypeId>,
    pub platforms: Vec<PlatformId>,
    pub regions: Vec<RegionId>,
    pub genres: Vec<GenreId>,
    pub engines: Vec<EngineId>,
    pub developers: Vec<DeveloperId>,
    pub publishers: Vec<PublisherId>,
    pub moderators: HashMap<ModeratorId, ModeratorRole>,
    pub created: Option<DateTime<Utc>>,
    pub assets: Assets,
    pub links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameId(String);

impl std::fmt::Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Names {
    pub international: String,
    pub japanese: Option<String>,
    pub twitch: Option<String>,
}

impl std::fmt::Display for Names {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.international)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameAbbreviation(String);

impl std::fmt::Display for GameAbbreviation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weblink(String);

impl std::fmt::Display for Weblink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseDate(String);

impl std::fmt::Display for ReleaseDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruleset {
    #[serde(rename = "show-milliseconds")]
    pub show_milliseconds: bool,
    #[serde(rename = "require-verification")]
    pub require_verification: bool,
    #[serde(rename = "require-video")]
    pub require_video: bool,
    #[serde(rename = "run-times")]
    pub run_times: Vec<String>,
    #[serde(rename = "default-time")]
    pub default_time: String,
    #[serde(rename = "emulators-allowed")]
    pub emulators_allowed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GametypeId(String);

impl std::fmt::Display for GametypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformId(String);

impl std::fmt::Display for PlatformId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionId(String);

impl std::fmt::Display for RegionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenreId(String);

impl std::fmt::Display for GenreId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineId(String);

impl std::fmt::Display for EngineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeveloperId(String);

impl std::fmt::Display for DeveloperId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherId(String);

impl std::fmt::Display for PublisherId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModeratorId(String);

impl std::fmt::Display for ModeratorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModeratorRole {
    #[serde(rename = "moderator")]
    Moderator,
    #[serde(rename = "super-moderator")]
    SuperModerator,
}

impl std::fmt::Display for ModeratorRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Moderator => write!(f, "moderator"),
            Self::SuperModerator => write!(f, "super-moderator"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    pub logo: Option<Asset>,
    #[serde(rename = "cover-tiny")]
    pub cover_tiny: Option<Asset>,
    #[serde(rename = "cover-small")]
    pub cover_small: Option<Asset>,
    #[serde(rename = "cover-medium")]
    pub cover_medium: Option<Asset>,
    #[serde(rename = "cover-large")]
    pub cover_large: Option<Asset>,
    pub icon: Option<Asset>,
    #[serde(rename = "trophy-1st")]
    pub trophy_1st: Option<Asset>,
    #[serde(rename = "trophy-2nd")]
    pub trophy_2nd: Option<Asset>,
    #[serde(rename = "trophy-3rd")]
    pub trophy_3rd: Option<Asset>,
    #[serde(rename = "trophy-4th")]
    pub trophy_4th: Option<Asset>,
    pub foreground: Option<Asset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    uri: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links(Vec<Link>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub rel: String,
    pub uri: String,
}

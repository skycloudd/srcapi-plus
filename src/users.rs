use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub names: Names,
    #[serde(rename = "supporterAnimation")]
    pub supporter_animation: bool,
    pub pronouns: Pronouns,
    pub weblink: Weblink,
    #[serde(rename = "name-style")]
    pub name_style: NameStyle,
    pub role: Role,
    pub signup: Option<DateTime<Utc>>,
    pub location: Location,
    pub twitch: Option<Social>,
    pub hitbox: Option<Social>,
    pub youtube: Option<Social>,
    pub twitter: Option<Social>,
    pub speedrunslive: Option<Social>,
    pub assets: Assets,
    pub links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(String);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Names {
    pub international: String,
    pub japanese: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pronouns(String);

impl std::fmt::Display for Pronouns {
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
pub struct NameStyle {
    pub style: String,
    #[serde(rename = "color-from")]
    pub color_from: Color,
    #[serde(rename = "color-to")]
    pub color_to: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub light: String,
    pub dark: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Banned,
    User,
    Trusted,
    Moderator,
    Admin,
    Programmer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub country: Country,
    pub region: Option<Region>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub names: Names,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub code: String,
    pub names: Names,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Social {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    pub icon: Icon,
    #[serde(rename = "supporterIcon")]
    pub supporter_icon: Option<SupporterIcon>,
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupporterIcon {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links(Vec<Link>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub rel: String,
    pub uri: String,
}

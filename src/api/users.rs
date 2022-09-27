use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Users_ {
    pub(crate) data: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User_ {
    pub(crate) data: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    names: Names,
    #[serde(rename = "supporterAnimation")]
    supporter_animation: bool,
    pronouns: String,
    weblink: String,
    #[serde(rename = "name-style")]
    name_style: NameStyle,
    role: Role,
    signup: String,
    location: Location,
    twitch: Option<Social>,
    hitbox: Option<Social>,
    youtube: Option<Social>,
    twitter: Option<Social>,
    speedrunslive: Option<Social>,
    assets: Assets,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
struct Names {
    international: String,
    japanese: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NameStyle {
    style: String,
    #[serde(rename = "color-from")]
    color_from: Color,
    #[serde(rename = "color-to")]
    color_to: Color,
}

#[derive(Debug, Serialize, Deserialize)]
struct Color {
    light: String,
    dark: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    Banned,
    User,
    Trusted,
    Moderator,
    Admin,
    Programmer,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    country: Country,
    region: Option<Region>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Country {
    code: String,
    names: Names,
}

#[derive(Debug, Serialize, Deserialize)]
struct Region {
    code: String,
    names: Names,
}

#[derive(Debug, Serialize, Deserialize)]
struct Social {
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Assets {
    icon: Icon,
    #[serde(rename = "supporterIcon")]
    supporter_icon: Option<SupporterIcon>,
    image: Image,
}

#[derive(Debug, Serialize, Deserialize)]
struct Icon {
    uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SupporterIcon {
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Links(Vec<Link>);

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    rel: String,
    uri: String,
}

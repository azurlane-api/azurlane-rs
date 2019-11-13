#![allow(missing_docs)]

#[derive(Deserialize, Debug, Clone)]
pub struct Names {
    pub en: Option<String>,
    pub cn: Option<String>,
    pub jp: Option<String>,
    pub kr: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Skin {
    pub title: Option<String>,
    pub image: Option<String>,
    pub chibi: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Stars {
    pub value: Option<String>,
    pub count: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Stat {
    pub name: Option<String>,
    pub image: Option<String>,
    pub value: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Stats {
    #[serde(rename = "100")]
    pub hundred: Option<Vec<Stat>>,
    #[serde(rename = "120")]
    pub hundred_twenty: Option<Vec<Stat>>,
    pub base: Option<Vec<Stat>>,
    pub retrofit100: Option<Vec<Stat>>,
    pub retrofit120: Option<Vec<Stat>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MiscellaneousData {
    pub link: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Miscellaneous {
    pub artist: Option<MiscellaneousData>,
    pub web: Option<MiscellaneousData>,
    pub pixiv: Option<MiscellaneousData>,
    pub twitter: Option<MiscellaneousData>,
    #[serde(rename="voiceActress")]
    pub voice_actress: Option<MiscellaneousData>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct Ship {
    pub wiki_url: String,
    pub id: Option<String>,
    pub names: Names,
    pub thumbnail: String,
    pub skins: Vec<Skin>,
    pub build_time: Option<String>,
    pub rarity: Option<String>,
    pub stars: Stars,
    pub class: Option<String>,
    pub nationality: Option<String>,
    pub nationality_short: Option<String>,
    pub hull_type: Option<String>,
    pub stats: Stats,
    pub miscellaneous: Miscellaneous,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct ShipResponse {
    pub status_code: u32,
    pub status_message: String,
    pub message: String,
    pub ship: Ship,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SmallShip {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct ShipsResponse {
    pub status_code: u32,
    pub status_message: String,
    pub message: String,
    pub ships: Vec<SmallShip>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Construction {
    pub time: String,
    #[serde(rename="wikiUrl")]
    pub wiki_url: String,
    pub ships: Vec<String>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct ConstructionResponse {
    pub status_code: u32,
    pub status_message: String,
    pub message: String,
    pub construction: Construction,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SmallShip {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ship {
    pub name: String,
    pub id: String,
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
#[serde(rename_all="camelCase")]
pub struct ShipsResponse {
    pub status_code: u32,
    pub status_message: String,
    pub message: String,
    pub ships: Vec<SmallShip>,
}
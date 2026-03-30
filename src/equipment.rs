use jiff::Timestamp;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EquipmentType {
    Brewer,
    Grinder,
}

#[derive(Debug, Deserialize)]
pub struct Equipment {
    id: u32,
    timestamp: Timestamp,

    pub name: String,
    pub equipment_type: EquipmentType,
    pub purchase_date: Timestamp,
    pub decommission_date: Option<Timestamp>,
    pub price_ct: u32,
}
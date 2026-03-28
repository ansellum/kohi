use jiff::Timestamp;
use serde::{Serialize, Deserialize};
use std::io::{self, Write};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
enum EquipmentType {
    Brewer,
    Grinder,
}

#[derive(Serialize, Deserialize)]
pub struct Equipment {
    id: u32,
    name: String,

    equipment_type: EquipmentType,
    purchase_date: Timestamp,
    decommission_date: Timestamp,
    price: u32,
    timestamp: Timestamp,
}

//pub fn equipment_from_json() -> Equipment {
    
//}
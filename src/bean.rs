use jiff::Timestamp;

enum RoastLevel {
    Light,
    Medium,
    Dark,
}

pub struct Bean {
    purchase_date: Timestamp,
    purchase_link: String,
    name: String,
    origin: Option<String>,
    process: Option<String>,
    roaster: String,
    roast_level: RoastLevel,
    roast_date: Option<Timestamp>,
}

pub fn create_bean(name: String, origin: Option<String>, ) {

}


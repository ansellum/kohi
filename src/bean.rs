use jiff::Timestamp;

enum RoastLevel {
    Light,
    Medium,
    Dark,
}

pub struct Origin {
    name: String,
    region: Vec<String>,
    farm: Option<String>,
    producer: Option<String>,
    varietal: Vec<String>,
    altitude: Option<String>,
}

pub struct Bean {
    purchase_date: Timestamp,
    roast_date: Timestamp,
    purchase_link: String,
    roaster: String,
    name: String,

    origin: Origin,
    roast_level: RoastLevel,
    tasting_notes: Vec<String>,

    weight: u16,
    price: u16, //in pennies
}
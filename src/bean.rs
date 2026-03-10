use jiff::Timestamp;

enum RoastLevel {
    Light,
    Medium,
    Dark,
}

pub struct Bean {
    name: String,
    origin: Option<String>,
    process: Option<String>,
    roaster: String,
    roast_level: RoastLevel,
    roast_date: Option<Timestamp>,
}


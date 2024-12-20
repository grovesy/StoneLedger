use chrono::{prelude::*, DateTime, Utc};

#[derive(Hash, Debug, Clone)]
pub struct Temporality {
    valid_from: DateTime::<Utc>,
    valid_to: DateTime::<Utc>,
}

impl Temporality {
    pub fn new() -> Self {
        Temporality { 
            valid_from: Utc::now(),
            valid_to: Utc.with_ymd_and_hms(9999, 01, 01, 0, 0, 0).unwrap(),
        }
    }

    pub fn valid_from(&self) -> DateTime<Utc> {
        self.valid_from
    }

    pub fn valid_to(&self) -> DateTime<Utc> {
        self.valid_to
    }
}
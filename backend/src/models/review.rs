use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Review {
    pub rating: i32,
    pub review: String,
}

impl Review {
    pub fn new(rating: i32, review: String) -> Self {
        Self { rating, review }
    }
}

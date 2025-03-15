use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static LISTINGS: OnceLock<Mutex<HashMap<u32, Listing>>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Listing {
    pub title: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub username: String,
}

pub fn get_listings() -> &'static Mutex<HashMap<u32, Listing>> {
    LISTINGS.get_or_init(|| Mutex::new(HashMap::new()))
}

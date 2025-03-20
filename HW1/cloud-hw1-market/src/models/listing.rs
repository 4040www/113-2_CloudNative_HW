use std::collections::{HashMap, BTreeMap};
use std::sync::{Mutex, OnceLock};

// Listings
static LISTINGS: OnceLock<Mutex<HashMap<u32, Listing>>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Listing {
    pub title: String,
    pub description: String,
    pub price: f64,
    pub username: String,
    pub creation_time: String,
}

pub fn get_listings() -> &'static Mutex<HashMap<u32, Listing>> {
    LISTINGS.get_or_init(|| Mutex::new(HashMap::new()))
}


// Categories
static CATEGORIES: OnceLock<Mutex<HashMap<String, BTreeMap<u32, String>>>> = OnceLock::new();  // 使用 BTreeMap 以便於排序

// #[derive(Debug, Clone)]
// pub struct Category {
//     pub name: String,
//     pub listings: BTreeMap<u32, String>,
// }

pub fn get_categories() -> &'static Mutex<HashMap<String, BTreeMap<u32, String>>> {
    CATEGORIES.get_or_init(|| Mutex::new(HashMap::new()))
}

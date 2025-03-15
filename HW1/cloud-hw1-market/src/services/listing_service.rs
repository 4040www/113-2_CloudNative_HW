use crate::models::listing;
use crate::models::user;
use std::sync::{Mutex, OnceLock};

static LISTING_ID_COUNTER: OnceLock<Mutex<u32>> = OnceLock::new();

pub fn create_listing(username: &str, title: &str, description: &str, price: f64, category: &str) -> String {
    let users = user::get_users();
    let users = users.lock().unwrap();

    if !users.contains(&username.to_lowercase()) {
        return "Error - unknown user".to_string();
    }

    let mut listings = listing::get_listings().lock().unwrap();
    let mut counter = LISTING_ID_COUNTER.get_or_init(|| Mutex::new(100001)).lock().unwrap();  // 初始值設定為 100001

    let listing_id = *counter;
    *counter += 1;

    let new_listing = listing::Listing {
        title: title.to_string(),
        description: description.to_string(),
        price,
        category: category.to_string(),
        username: username.to_string(),
    };

    listings.insert(listing_id, new_listing);

    listing_id.to_string()
}

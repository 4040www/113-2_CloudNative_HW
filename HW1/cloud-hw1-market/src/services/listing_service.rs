use crate::models::listing;
use crate::models::user;
use crate::utils::utils::get_formatted_time;

use std::sync::{Mutex, OnceLock, MutexGuard};
use std::collections::BTreeMap;

// global counter for listing IDs
static LISTING_ID_COUNTER: OnceLock<Mutex<u32>> = OnceLock::new();

pub fn create_listing(username: &str, title: &str, description: &str, price: f64, category: &str) -> String {
    let users = user::get_users();
    let users = users.lock().unwrap();

    if !users.contains(username) {
        return "Error - unknown user".to_string();
    }

    // create a new listing
    let mut listings = listing::get_listings().lock().unwrap();
    let mut counter = LISTING_ID_COUNTER.get_or_init(|| Mutex::new(100001)).lock().unwrap();  // 初始值設定為 100001

    let listing_id = *counter;
    *counter += 1;

    let create_time = get_formatted_time();

    let new_listing = listing::Listing {
        title: title.to_string(),
        description: description.to_string(),
        price,
        username: username.to_string(),
        creation_time: create_time,
    };

    listings.insert(listing_id, new_listing);

    // update categories
    let mut categories = listing::get_categories().lock().unwrap();
    let category_ = categories.entry(category.to_string()).or_insert(BTreeMap::new());
    category_.insert(listing_id, category.to_string());

    listing_id.to_string()
}

pub fn delete_listing(username: &str, listing_id: &str) -> String {
    let users = user::get_users();
    let users = users.lock().unwrap();

    if !users.contains(username) {
        return "Error - unknown user".to_string();
    }

    let mut listings = listing::get_listings().lock().unwrap();

    let listing_id: u32 = match listing_id.parse() {
        Ok(id) => id,
        Err(_) => return "Error - invalid listing ID".to_string(),
    };

    // find the listing and delete it
    match listings.get_mut(&listing_id) {
        Some(listing) => {
            if listing.username != username {
                return "Error - listing owner mismatch".to_string();
            }

            // delete the listing
            listings.remove(&listing_id);

            // update categories
            let mut categories = listing::get_categories().lock().unwrap();
            for category in categories.values_mut() {
                category.remove(&listing_id);
            }

            return "Success".to_string();
        }
        None => return "Error - listing does not exist".to_string(), // listing 不存在
    }
}

pub fn get_listing(username: &str, listing_id: &str) -> String {
    let users = user::get_users();
    let users: MutexGuard<_> = users.lock().unwrap();

    if !users.contains(username) {
        return "Error - unknown user".to_string();
    }

    // find the listing
    let listing_id: u32 = match listing_id.parse() {
        Ok(id) => id,
        Err(_) => return "Error - invalid listing ID".to_string(),
    };

    let listings = listing::get_listings();
    let listings: MutexGuard<_> = listings.lock().unwrap();

    let categories = listing::get_categories();
    let categories: MutexGuard<_> = categories.lock().unwrap();

    // get the listing details and return
    match listings.get(&listing_id) {
        Some(listing) => {
            let mut category_name = "Unknown".to_string();
            for (category, listing_ids) in categories.iter() {
                if listing_ids.contains_key(&listing_id) {
                    category_name = category.clone();
                    break;
                }
            }

            format!(
                "{}|{}|{}|{}|{}|{}",
                listing.title, listing.description, listing.price, listing.creation_time, category_name, listing.username
            )
        }
        None => "Error - not found".to_string(),
    }
}

pub fn get_category(username: &str, category: &str) -> String {
    let users = user::get_users();
    let users = users.lock().unwrap();

    if !users.contains(username) {
        return "Error - unknown user".to_string();
    }

    let categories = listing::get_categories();
    let categories: MutexGuard<_> = categories.lock().unwrap();

    // 檢查是否存在該 category
    if let Some(category_data) = categories.get(category) {
        let listings = listing::get_listings();
        let listings: MutexGuard<_> = listings.lock().unwrap();

        // 取得該 category 下所有 listing_id
        let sorted_listings: Vec<_> = category_data.iter().collect();
        let mut response = String::new();

        for (listing_id, _) in sorted_listings.iter().rev() {
            // 根據 listing_id 查詢 listings
            if let Some(listing) = listings.get(listing_id) {
                response.push_str(&format!(
                    "{}|{}|{}|{}|{}|{}\n", 
                    listing.title, 
                    listing.description, 
                    listing.price, 
                    listing.creation_time,
                    category,
                    listing.username
                ));
            }
        }
        response.trim_end().to_string() // 去除最後的換行符號
    } else {
        "Error - category not found".to_string()
    }
}



pub fn get_top_category(username: &str) -> String {
    let users = user::get_users();
    let users = users.lock().unwrap();

    if !users.contains(username) {
        return "Error - unknown user".to_string();
    }

    let categories = listing::get_categories();
    let categories: MutexGuard<_> = categories.lock().unwrap();

    let mut category_count: BTreeMap<String, usize> = BTreeMap::new();
    for (category, listing_ids) in categories.iter() {
        category_count.insert(category.clone(), listing_ids.len());
    }

    match category_count.iter().max_by_key(|&(_, count)| count) {
        Some((category, _)) => category.clone(),
        None => "Error - no categories available".to_string(),
    }
}
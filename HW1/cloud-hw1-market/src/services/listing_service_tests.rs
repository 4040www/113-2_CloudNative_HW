use std::sync::Mutex;

use crate::services::listing_service::*;
use crate::models::user;
use crate::models::listing;

fn setup_user(username: &str) {
    let users = user::get_users();
    let mut users = users.lock().unwrap();
    users.insert(username.to_string(), true);
}

fn clear_all() {
    listing::get_listings().lock().unwrap().clear();
    listing::get_categories().lock().unwrap().clear();
    user::get_users().lock().unwrap().clear();
}

#[test]
fn test_create_listing_success() {
    clear_all();
    setup_user("alice");

    let result = create_listing("alice", "Item A", "Nice one", 123.0, "Gadgets");
    assert!(result.parse::<u32>().is_ok(), "Should return a listing ID");
}

#[test]
fn test_create_listing_unknown_user() {
    clear_all();
    let result = create_listing("bob", "Item B", "Unknown", 100.0, "Books");
    assert_eq!(result, "Error - unknown user");
}

#[test]
fn test_delete_listing_success() {
    clear_all();
    setup_user("alice");

    let id = create_listing("alice", "Phone", "Desc", 500.0, "Electronics");
    let result = delete_listing("alice", &id);
    assert_eq!(result, "Success");
}

#[test]
fn test_delete_listing_wrong_user() {
    clear_all();
    setup_user("alice");
    setup_user("bob");

    let id = create_listing("alice", "Phone", "Desc", 500.0, "Electronics");
    let result = delete_listing("bob", &id);
    assert_eq!(result, "Error - listing owner mismatch");
}

#[test]
fn test_delete_listing_invalid_id() {
    clear_all();
    setup_user("alice");

    let result = delete_listing("alice", "invalid_id");
    assert_eq!(result, "Error - listing does not exist");
}

#[test]
fn test_get_listing_success() {
    clear_all();
    setup_user("alice");

    let id = create_listing("alice", "Book", "Used", 50.0, "Books");
    let result = get_listing("alice", &id);

    assert!(result.contains("Book|Used|"), "Should contain listing details");
}

#[test]
fn test_get_listing_not_found() {
    clear_all();
    setup_user("alice");

    let result = get_listing("alice", "999999");
    assert_eq!(result, "Error - not found");
}

#[test]
fn test_get_category_success() {
    clear_all();
    setup_user("alice");

    create_listing("alice", "Item1", "desc1", 10.0, "Shoes");
    create_listing("alice", "Item2", "desc2", 20.0, "Shoes");

    let result = get_category("alice", "Shoes");
    assert!(result.contains("Item1") && result.contains("Item2"));
}

#[test]
fn test_get_category_not_found() {
    clear_all();
    setup_user("alice");

    let result = get_category("alice", "Nonexist");
    assert_eq!(result, "Error - category not found");
}

#[test]
fn test_get_top_category() {
    clear_all();
    setup_user("alice");

    create_listing("alice", "Item1", "desc1", 10.0, "Shoes");
    create_listing("alice", "Item2", "desc2", 10.0, "Shoes");
    create_listing("alice", "Item3", "desc3", 10.0, "Electronics");

    let result = get_top_category("alice");
    assert_eq!(result, "Shoes");
}

#[test]
fn test_get_top_category_no_categories() {
    clear_all();
    setup_user("alice");

    let result = get_top_category("alice");
    assert_eq!(result, "Error - unknown user");
}

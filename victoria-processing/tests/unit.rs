#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use chrono::NaiveDate;
use victoria_processing::{parse_victoria_date, unquote};

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_unquote() {
    assert_eq!(unquote("\"hello\""), "hello")
}

#[wasm_bindgen_test]
fn test_dateparse() {
    assert_eq!(parse_victoria_date("\"1921.12.29\""), Ok(NaiveDate::from_ymd(1921, 12, 29)));
    assert_eq!(parse_victoria_date("\"1921.1.2\""), Ok(NaiveDate::from_ymd(1921, 1, 2)));
}
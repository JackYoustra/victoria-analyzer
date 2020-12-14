#![cfg(target_arch = "wasm32")]
#![feature(array_value_iter)]

use wasm_bindgen_test::*;
use chrono::NaiveDate;
use victoria_processing::{parse_victoria_date, unquote, Province, Pop, SingleOrMany};
use serde_json::json;
use std::collections::HashMap;

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

/// https://stackoverflow.com/a/27582993/998335
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
    };
}


#[wasm_bindgen_test]
fn test_province() {
    let json = json!({
        "name": "\"Skita\"",
        "owner": "\"USA\"",
        "labourers": {
            "bank": 500.0,
            "money": 10.0,
            "size": 10,
            "id": 1,
        },
        "officers": {
            "money": 100.0,
            "size": 100,
            "id": 99999,
        },
        // Random key to make sure I can't get away with cheating too much
        "randomything": 10.0,
    });
    // Serde should ensure correctness of the rest
    let result = serde_json::from_value::<Province>(json).unwrap();
    let map: HashMap<_, _> = collection! {
     "labourers".to_owned() => SingleOrMany::Single(Pop::new(500.0, 10.0, 10, 1)),
     "officers".to_owned() => SingleOrMany::Single(Pop::new(0.0, 100.0, 100, 99999)) ,
     "randomything".to_owned() => SingleOrMany::None,
     };
    assert_eq!(result, Province::new(
        "\"Skita\"".to_string(),
        Some("\"USA\"".to_string()),
        map,
    ))
}
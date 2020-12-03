use wasm_bindgen_test::*;
use victoria_processing::{Save, parse_save};

#[macro_use]
extern crate matches;

#[wasm_bindgen_test]
fn file_test() {
    let ukgame = include_str!("UK Warround 2.v2");
    let save_value = parse_save(ukgame);
    assert_matches!(save_value, Ok(_));
    let save_result = Save::new(save_value.unwrap());
    assert_matches!(save_result, Ok(_));
}
use wasm_bindgen_test::*;
use victoria_processing::{Save, Node, parse_save};
use wasm_bindgen::JsValue;

#[macro_use]
extern crate matches;

#[wasm_bindgen_test]
fn file_test() {
    let ukgame = include_str!("UK Warround 2.v2");
    let save_value = parse_save(ukgame);
    assert_matches!(save_value, Ok(Node::List(_)));
    let mut save_value = save_value.unwrap();
    save_value.raise();
    let save_result = Save::new(save_value.clone());
    assert_matches!(save_result, Ok(_));
    let save_result = save_result.unwrap();
}
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
    let forex_diag = save_result.forex_diagnostic();
    assert_eq!(forex_diag.len(), 221);
    assert_eq!(forex_diag["ENG"], 520784621.5579901);
    assert_eq!(forex_diag["USA"], 802537723.3783101);
    // Eventually, want single-states to have balances reflected here
    assert_eq!(forex_diag["CRE"], 0.0);
    // Because we're not counting single-states, we want to make sure we don't accidentally include single states++_
    assert_eq!(forex_diag.values().filter(|x| x == &&0.0).count(), 181);

    let forex = save_result.forex_position();
    let (treasury, wealth_by_state) = &forex["USA"];
    assert_eq!(treasury.clone(), 29636.93976);
    console_log!("{:?}", wealth_by_state.keys());
    let (factories, provinces) = &wealth_by_state[&1];
    console_log!("{:?}", factories.keys());
    let factory_wealth = factories[&"hello"];
    assert_eq!(factory_wealth, 100.0);
    console_log!("{:?}", provinces.keys());
    let province = &provinces[&"hello"];
    console_log!("{:?}", province.keys());
    let pop = province[&"labourer"];
    assert_eq!(pop, 100.0);
}
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

    let forex = save_result.forex_position();
    // Because we're counting all states, we want to make sure everyone who has a state (in existence) is counted
    // console_log!("{:?}", forex.iter().filter(|(name, (x, _))| x == &0.0).map(|(name, _)| name).collect::<Vec<&&str>>());
    // Should be eight bankrupt countries
    assert_eq!(forex.values().filter(|(x, _)| x == &0.0).count(), 8);
    // Make sure single state is counted
    // Country: LUA: Single(State { buildings: Many([Building { name: "\"liquor_distillery\"", money: 1000000.0 }, Building { name: "\"regular_clothes_factory\"", money: 693054.63766 }]), savings: 542.49167, interest: 1840.22009, id: StateID { id: 1091, state_type: 47 }, province_ids: [1356, 1362] }
    let (treasury, wealth_by_state) = &forex["LUA"];
    assert_eq!(treasury.clone(), 7538.23181);
    // Just one state - lots of single tests here
    let (factories, provinces) = &wealth_by_state[&1091];
    // Two factories
    let factory_wealth = factories[&"\"liquor_distillery\""];
    assert_eq!(factory_wealth, 1000000.0);
    let factory_wealth = factories[&"\"regular_clothes_factory\""];
    assert_eq!(factory_wealth, 693054.63766);
    // Two provinces
    let province = &provinces[&"\"Salavan\""];
    let pop = province["craftsmen"];
    assert_eq!(pop, 395.94034);
    let province = &provinces[&"\"Luang"];
    let pop = province["capitalists"];
    assert_eq!(pop, 244017.08801);

    // And now do it on a GP
    let (treasury, wealth_by_state) = &forex["USA"];
    assert_eq!(treasury.clone(), 5333614.26907);
    let (factories, provinces) = &wealth_by_state[&1293];
    let factory_wealth = factories[&"\"furniture_factory\""];
    assert_eq!(factory_wealth, 2000000.0);
    // ID #1
    let province = &provinces[&"\"Sitka\""];
    let pop = province["labourers"];
    assert_eq!(pop, 533502.6074);
}
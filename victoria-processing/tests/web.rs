//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use victoria_processing::{unquote, save_parser, Node, parse_victoria_date};
use chrono::NaiveDate;

#[macro_use]
extern crate matches;

// wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
fn manual_test() {
    let date_string = "date=1921.12.29";
    let date_expression = Node::Line(("date", vec![Node::Leaf("1921.12.29")]));
    let unit_string = "unit_cost=
    {
    ammunition=96.29
    canned_food=148.48
    }";
    let unit_expression =
        Node::Line(("unit_cost", vec![
            Node::Line(("ammunition", vec![Node::Leaf("96.29")])),
            Node::Line(("canned_food", vec![Node::Leaf("148.48")])),
        ]));
    let employees_string = "employees=
    {

        {
            próvince_pop_id=
            {
            province_id=1
            index=0
            type=9
            }
            count=34256
        }
    }";
    let employees_expression =
        Node::Line(("employees", vec![
            Node::List(vec![
                Node::Line(("próvince_pop_id", vec![
                    Node::Line(("province_id", vec![Node::Leaf("1")])),
                    Node::Line(("index", vec![Node::Leaf("0")])),
                    Node::Line(("type", vec![Node::Leaf("9")])),
                ])),
                Node::Line(("count", vec![Node::Leaf("34256")])),
            ]),
        ]));
    assert_eq!(save_parser::entry(date_string), Ok(date_expression.clone()));
    assert_eq!(save_parser::entry(unit_string), Ok(unit_expression.clone()));
    assert_eq!(save_parser::save(unit_string), Ok(vec![unit_expression.clone()]));
    assert_eq!(save_parser::entry(employees_string), Ok(employees_expression.clone()));
    assert_eq!(save_parser::save(employees_string), Ok(vec![employees_expression.clone()]));
    let combined_expression = vec![
        date_expression,
        unit_expression,
        employees_expression
    ];
    assert_eq!(save_parser::save(&(date_string.to_owned() + "\n" + unit_string + "\n" + employees_string)), Ok(combined_expression));
}

#[wasm_bindgen_test]
fn file_test() {
    let ukgame = include_str!("UK Warround 2.v2");
    assert_matches!(save_parser::save(ukgame), Ok(_));
}


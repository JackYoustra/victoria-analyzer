#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use victoria_processing::{save_parser, Node, parse_save};
use chrono::NaiveDate;

#[macro_use]
use serde_json;

#[wasm_bindgen_test]
fn manual_test() {
    let date_string = "date=1921.12.29";
    let date_expression = Node::SingleElementLine(("date", "1921.12.29"));
    let unit_string = "unit_cost=
    {
    ammunition=96.29
    ammunition=22.50
    canned_food=148.48
    }";
    let unit_expression =
        Node::Line(("unit_cost", vec![
            Node::SingleElementLine(("ammunition", "96.29")),
            Node::SingleElementLine(("ammunition", "22.50")),
            Node::SingleElementLine(("canned_food", "148.48")),
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
                    Node::SingleElementLine(("province_id", "1")),
                    Node::SingleElementLine(("index", "0")),
                    Node::SingleElementLine(("type", "9")),
                ])),
                Node::SingleElementLine(("count", "34256")),
            ]),
        ]));
    assert_eq!(save_parser::entry(date_string), Ok(date_expression.clone()));
    assert_eq!(save_parser::entry(unit_string), Ok(unit_expression.clone()));
    assert_eq!(save_parser::save(unit_string), Ok(vec![unit_expression.clone()]));
    assert_eq!(save_parser::entry(employees_string), Ok(employees_expression.clone()));
    assert_eq!(save_parser::save(employees_string), Ok(vec![employees_expression.clone()]));
    let combined_expression = Node::List(vec![
        date_expression,
        unit_expression,
        employees_expression
    ]);
    let result = date_string.to_owned() + "\n" + unit_string + "\n" + employees_string;
    let result = parse_save(&result).unwrap();
    assert_eq!(result, combined_expression);
    assert_eq!(result.to_json(), serde_json::json!({
        "date": "1921.12.29",
        "unit_cost": {
            "ammunition": [96.29, 22.50],
            "canned_food": 148.48
        },
        "employees": {
            "list": {
                "próvince_pop_id": {
                    "province_id": 1,
                    "index": 0,
                    "type": 9
                },
                "count": 34256
            }
        }
    }));
}

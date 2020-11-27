mod utils;

use wasm_bindgen::prelude::*;
use peg;
use chrono::NaiveDate;
use std::collections::BTreeMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

struct Product<'a> {
    name: &'a str,
    world_quantity: f64,
    price: f64,
    price_history: Vec<f64>,
    // assert discovered good true
}

struct WorldMarket {

}

struct Save<'a> {
    date: NaiveDate,
    player_tag: &'a str,
    flags: Vec<&'a str>,

}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node<'a> {
    Line((&'a str, Vec<Node<'a >>)),
    List(Vec<Node<'a >>),
    Leaf(&'a str),
}

peg::parser! {
    pub grammar save_parser() for str {
        rule _() = [' ' | '\n' | '\r' | '\t']*
        rule atomic() = quiet!{['a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '"' ]+}
            / expected!("atom")

        rule id() ->  &'input str
         = identifier:$(atomic()) "=" { identifier }

        rule list_element() -> Node<'input>
        = e:entry() { e }
         / a:$(atomic()) _ { Node::Leaf(a) }

        rule list() -> Vec<Node<'input>>
        = a:list_element()* { a }

        rule line() -> Node<'input>
         = identifier:id() leafsymbol:$(atomic()) _ { Node::Line((identifier, vec![Node::Leaf(leafsymbol)])) }

        pub rule entry() -> Node<'input>
        // parse normal lines
         = current:line() { current }
         // parse meaningful trees
         / identifier:id() _ "{" _ e:entry()* "}" _ { Node::Line((identifier, e)) }
         // parse named lists
         / identifier:id() _ "{" _ e:list() "}" _ { Node::Line((identifier, e)) }
         // parse anon lysts
         / _ "{" _ e:list() _ "}" _ { Node::List(e) }

       pub rule save() -> Vec<Node<'input>>
        = e:entry()* { e }
    }
}

pub fn greet() {
    alert("Hello, victoria-processing!");
}

#[wasm_bindgen]
pub fn process_save(savetext: &str) -> Option<String> {
    return match save_parser::entry(savetext) {
        Ok(_) => None,
        Err(E) => Some(E.to_string()),
        _ => None,
    }
}
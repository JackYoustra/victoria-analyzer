mod utils;

use wasm_bindgen::prelude::*;
use peg;
use chrono::NaiveDate;
use std::num::ParseIntError;
use core::fmt;
use std::error;

use serde_json;
use crate::ParseError::MissingNode;

use lazy_static::lazy_static; // 1.3.0
use regex::Regex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// Regexes
lazy_static! {
    static ref COUNTRY_TAG: Regex = Regex::new(r"^[A-Z]{3}$").unwrap();
}

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

#[derive(Debug, Eq, PartialEq, Clone)]
struct Building<'a> {
    name: &'a str,
    //money: f64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State<'a> {
    buildings: &'a [Building<'a>],
}

impl<'a> State<'a> {
    fn new(list: &'a Node<'a>) -> Result<State, ParseError> {
        Err(MissingNode)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Country<'a> {
    states: Vec<State<'a>>,
}

impl<'a> Country<'a> {
    fn new(list: Node<'a>) -> Result<Country, ParseError> {
        if let Node::List(props) = list {
            let mut states = None::<Vec<State>>;
            let error = props.iter().filter_map(|x| -> Option<ParseError> {
                match x {
                    Node::Line(("state", vec)) if states == None => {
                        let results: Result<Vec<State>, _> = vec.iter().map(State::new).collect();
                        match results {
                            Ok(stateResults) => {
                                states = Some(stateResults);
                                None
                            },
                            Err(bad) => Some(bad)
                        }
                    },

                    _ => None,
                }
            }).next();
            return match error {
                Some(err) => Err(err.clone()),
                None => Ok(Country {
                    // states: states.unwrap(),
                    states: vec![],
                })
            }
        }
        Err(MissingNode)
    }
}

#[derive(Debug)]
pub struct Save<'a> {
    date: NaiveDate,
    player_tag: &'a str,
    // flags: Vec<&'a str>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseError {
    InvalidDate,
    Integer(ParseIntError),
    MissingNode,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::MissingNode => write!(f, "Missing node"),
            ParseError::InvalidDate =>
                write!(f, "Invalid date"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            ParseError::Integer(ref e) =>
                e.fmt(f)
                //write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseError::InvalidDate | ParseError::MissingNode => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            ParseError::Integer(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::Integer(err)
    }
}

// Until rust gets negative slice semantics, have to make do with this
pub fn unquote(thing: &str) -> &str {
    assert_eq!(thing.chars().nth(0), Some('"'));
    assert_eq!(thing.chars().nth(thing.len() - 1), Some('"'));
    return &thing[1 ..= thing.len() - 2];
}

pub fn parse_victoria_date(text: &str) -> Result<NaiveDate, ParseError> {
    let text = unquote(text);
    let textiter = text.char_indices();
    let dots: Vec<usize> = textiter.filter_map(|(x, y)| match y {
        '.' => Some(x),
        _ => None,
    }).take(2).collect();
    match (text[0..dots[0]].parse(),
           text[(dots[0] + 1)..dots[1]].parse(),
           text[(dots[1] + 1)..].parse(),
    ) {
        (Ok(y), Ok(m), Ok(d)) => {
            match NaiveDate::from_ymd_opt(y, m, d) {
                Some(date) => Ok(date),
                None => Err(ParseError::InvalidDate),
            }
        },
        (y, m, d) => {
            Err([y.err(), m.err(), d.err()]
                .iter()
                .find_map(|x| x.clone())
                .map_or(ParseError::InvalidDate, |x| ParseError::Integer(x)))
        },
    }
}

impl<'a> Save<'a> {
    pub fn new(list: Node<'a>) -> Result<Save, ParseError> {
        return Err(MissingNode);
        if let Node::List(thing) = list {
            let mut date = None::<NaiveDate>;
            let mut player_tag = None::<&'a str>;
            let error = thing.iter().filter_map( |x| -> Option<ParseError> {
                match x {
                    Node::Line(("date", vec)) if date == None => {
                        match vec.as_slice() {
                            [Node::Leaf(raw_date)] => {
                                match parse_victoria_date(raw_date) {
                                    Ok(good) => {
                                        date = Some(good);
                                        return None
                                    }
                                    Err(bad) => {
                                        return Some(bad)
                                    }
                                }
                            }
                            _ => None
                        }
                    },
                    Node::Line(("player", vec)) if player_tag == None => {
                        match vec.as_slice() {
                            [Node::Leaf(tag)] => {
                                player_tag = Some(unquote(tag));
                                None
                            }
                            _ => None
                        }
                    },
                    // All countries are three-letter words
                    Node::Line((rootkey, vec)) => {
                        if COUNTRY_TAG.is_match(rootkey) {

                        }
                        None
                    },

                    _ => None,
                }
            }).next();
            return match error {
                Some(err) => Err(err),
                None => Ok(Save {
                   date: date.unwrap(),
                   player_tag: player_tag.unwrap(),
               })
            }
        }
        Err(MissingNode)
    }
}

// https://stackoverflow.com/questions/32571441/what-is-the-difference-between-storing-a-vec-vs-a-slice
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node<'a> {
    Line((&'a str, Vec<Node<'a >>)),
    SingleElementLine((&'a str, &'a str)),
    List(Vec<Node<'a >>),
    Leaf(&'a str),
}

impl<'a> Node<'a> {
    fn insert_or_listify(name: &'a str, object: &serde_json::Value, map: &mut serde_json::Map<String, serde_json::Value>, seen: &mut Vec<&'a str>) {
        if let Some(prior) = map.get(name) {
            // if we already have an entry in the map for this element,
            // convert it to a list of this element with the name as a key
            // for now, means we can't invert unless we make this nicer
            if seen.contains(&name) {
                // append to list
                if let Some(serde_json::Value::Array(elements)) = map.get_mut(name) {
                    elements.push(object.clone());
                } else {
                    unreachable!()
                }
            } else {
                // create list
                seen.push(name);
                map.insert(name.to_string(), serde_json::Value::Array(vec![prior.clone(), object.clone()]));
            }
        } else {
            map.insert(name.to_string(), object.clone());
        }
    }

    // convert a function to serde's json
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Node::Line((_, arr)) | Node::List(arr) => {
                // Object if any element has a child
                // List if none do
                // Undefined if both
                if let Some(thing) = arr.first() {
                    match thing {
                        // List
                        Node::Leaf(_) => serde_json::Value::Array(arr.iter().map(|x| x.to_json()).collect()),
                        // Object
                        _ => {
                            let mut map = serde_json::Map::new();
                            let mut stuff = vec![];
                            for element in arr.iter() {
                                match element {
                                    Node::Line((name, innerLineItems)) => {
                                        Node::insert_or_listify(name, &Node::List(innerLineItems.clone()).to_json(), &mut map, &mut stuff);
                                    }
                                    l @ Node::List(_) => {
                                        Node::insert_or_listify("list", &l.to_json(), &mut map, &mut stuff);
                                    }
                                    Node::SingleElementLine((name, object)) => {
                                        Node::insert_or_listify(name, &Node::Leaf(object).to_json(), &mut map, &mut stuff);
                                    }
                                    Node::Leaf(name) => {
                                        unreachable!();
                                        Node::insert_or_listify(name, &serde_json::Value::Null, &mut map, &mut stuff);
                                    }
                                }
                            }
                            serde_json::Value::Object(map)
                        }
                    }
                } else {
                    // just return empty
                    serde_json::Value::Array(vec![])
                }
            }
            Node::Leaf(str) | Node::SingleElementLine((_, str)) => {
                match str.parse::<serde_json::Number>() {
                    Ok(val) => serde_json::Value::Number(val),
                    _ => serde_json::Value::String(str.to_string())
                }
            }
        }
    }
}

peg::parser! {
    pub grammar save_parser() for str {
        rule _() = [' ' | '\n' | '\r' | '\t']*
        rule atomic()
         = quiet!{ (![' ' | '\n' | '\r' | '\t' | '=' | '}' | '{'] [_])+ }
            / expected!("atom")

        rule id() ->  &'input str
         = identifier:$(atomic()) "=" { identifier }

        rule list_element() -> Node<'input>
        = e:entry() { e }
         / a:$(atomic()) _ { Node::Leaf(a) }

        rule list() -> Vec<Node<'input>>
        = a:list_element()* { a }

        rule line() -> Node<'input>
         = identifier:id() leafsymbol:$(atomic()) _ { Node::SingleElementLine((identifier, leafsymbol)) }

        pub rule entry() -> Node<'input>
        // parse normal lines
         = current:line() { current }
         // parse meaningful trees
         / identifier:id() _ "{" _ e:entry()* "}" _ { Node::Line((identifier, e)) }
         // parse named lists
         / identifier:id() _ "{" _ e:list() "}" _ { Node::Line((identifier, e)) }
         // parse anon lists
         / _ "{" _ e:list() _ "}" _ { Node::List(e) }

       pub rule save() -> Vec<Node<'input>>
       // dumb trailing curly brace
        = e:entry()* "}"? _ { e }
    }
}

pub fn parse_save(savetext: &str) -> Result<Node, peg::error::ParseError<peg::str::LineCol>> {
    match save_parser::save(savetext) {
        Ok(e) => Ok(Node::List(e)),
        Err(e) => Err(e),
    }
}

#[wasm_bindgen]
pub fn process_save(savetext: &str) -> Option<String> {
    return match save_parser::entry(savetext) {
        Ok(_) => None,
        Err(E) => Some(E.to_string()),
    }
}
#![feature(drain_filter)]

mod utils;

use wasm_bindgen::prelude::*;
use peg;
use chrono::NaiveDate;
use std::num::ParseIntError;
use std::error;
use std::fmt;

use serde_json;
use serde_with::{ serde_as, DefaultOnError };
use crate::ParseError::MissingNode;

use lazy_static::lazy_static; // 1.3.0
use regex::Regex;
use serde::{Deserializer, Deserialize, de};
use serde_json::{Error, Value};
use web_sys::console;
use std::collections::HashMap;
use serde::export::PhantomData;
use crate::utils::set_panic_hook;
use std::collections::hash_map::RandomState;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// Regexes
lazy_static! {
    static ref COUNTRY_TAG: Regex = Regex::new(r"^[A-Z]{3}$").unwrap();
    static ref PROVINCE_TAG: Regex = Regex::new(r"^[0-9]*$").unwrap();
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

#[derive(Deserialize, Debug, PartialEq)]
pub struct Pop {
    /// Presumably the money deposited in the national bank
    #[serde(default)]
    bank: f64,
    /// Presumably the money on-hand
    money: f64,
    /// The pop size
    size: i64,
    /// The pop ID
    id: i32,
}

impl Pop {
    pub fn new(bank: f64, money: f64, size: i64, id: i32) -> Self {
        Pop { bank, money, size, id }
    }
}

#[serde_as]
#[derive(Deserialize, Debug, PartialEq)]
pub struct Province {
    name: String,
    #[serde(default)]
    owner: Option<String>,
    /// Small hack: make the remainder pops.
    /// This, shockingly, actually works for any subfield we can think of,
    /// so it's actually the magic backtracking we were looking for all along
    #[serde(flatten)]
    #[serde_as(as="HashMap<DefaultOnError, DefaultOnError>")]
    pops: HashMap<String, Option<Pop>>,
}

impl Province {
    pub fn new(name: String, owner: Option<String>, pops: HashMap<String, Option<Pop>, RandomState>) -> Self {
        Province { name, owner, pops }
    }
}

#[derive(Deserialize, Debug)]
struct Building {
    #[serde(rename = "building")]
    name: String,
    money: f64,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct StateID {
    #[serde_as(as="DefaultOnError")]
    id: i32,
    #[serde(rename = "type")]
    #[serde_as(as="DefaultOnError")]
    state_type: i32,
}

/// A state owned by a country
#[derive(Deserialize, Debug)]
struct State {
    #[serde(rename = "state_buildings", default)]
    #[serde_as(as="Vec<DefaultOnError>")]
    buildings: Vec<Building>,
    // What are these?
    #[serde(default)]
    #[serde_as(as="DefaultOnError")]
    savings: f64,
    #[serde(default)]
    #[serde_as(as="DefaultOnError")]
    interest: f64,
    id: StateID,
    #[serde(rename = "provinces")]
    #[serde_as(as="DefaultOnError")]
    province_ids: Vec<i32>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct Country {
    tax_base: f64,
    // Don't count single-state countries rn
    #[serde(rename="state", default)]
    #[serde_as(as="DefaultOnError")]
    states: Vec<State>,
}

#[wasm_bindgen]
#[derive(Deserialize, Debug)]
pub struct Save {
    #[serde(deserialize_with = "vicky_date_serialize_serde")]
    date: NaiveDate,
    #[serde(rename = "player")]
    player_tag: String,
    // USA: Country,
    /// Hack:
    /// we know we want all aliases that are country tags,
    /// so we'll accept all all uppercase sequences of characters of size two or three
    /// (26^2 + 26^3) = 18252. Not great. I actually tried this and it killed the compiler. Sad!
    /// The problem is around line 1168 on serde-rs's de.rs. It does explicit checking, not pattern
    /// matching against valid rust patterns (we could use that to our advantage as we did with the
    /// PEG parser). Additionally, it wouldn't populate a hashmap like we want - just a vec.
    /// This is surmountable (can infer country from other tags) but irrelevant because we can't actually do that.
    /// Solution: create an artificial countries tag somewhere else to do what we want.
    countries: HashMap<String, Country>,

    /// Same hack as countries
    provinces: HashMap<i32, Province>,
}

impl Save {
    /// Just return country -> treasury, wealth by state -> wealth by factory / pop (per province)
    pub fn forex_position(&self) -> HashMap<&str, (f64, HashMap<i32, (HashMap<&str, f64>, HashMap<&str, HashMap<&str, f64>>)>)> {
        self.countries.iter().map(|(name, country)| {
            (name.as_str(), (country.tax_base, country.states.iter()
                .map(|state| {
                    (state.id.id , (
                             state.buildings.iter().map(|building| (building.name.as_str(), building.money)).collect::<HashMap<&str, f64>>(),
                            state.province_ids.iter()
                            .map(|x| self.provinces.get(x).unwrap())
                            .filter(|x| x.owner.as_ref().map(|unwrapper| unwrapper) == Some(name))
                            .map(|x| {
                                (x.name.as_str(), x.pops.iter().filter_map(|(title, pop)| -> Option<(&str, f64)> {
                                    pop.as_ref().map(|x| (title.as_str(), x.bank + x.money))
                                }).collect::<HashMap<&str, f64>>())
                            }).collect::<HashMap<&str, HashMap<&str, f64>>>()
                         ))
                    }
                ).collect()))
        }).collect()
    }

    /// Just return country -> treasury + wealth by state. For testing purposes
    pub fn forex_diagnostic(&self) -> HashMap<&str, f64> {
        self.countries.iter().map(|(name, country)| {
            log!("{:?}", country.states);
            (name.as_str(), country.states.iter()
                .fold(0.0, |acc, x| acc + x.buildings.iter()
                    .fold(0.0, |buildingacc, building| buildingacc + building.money) ) )
        }).collect()
    }
}

fn vicky_date_serialize_serde<'de, D>(
    deserializer: D,
) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_victoria_date(&*s).map_err(serde::de::Error::custom)
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

impl Save {
    pub fn new(list: Node) -> Result<Save, Error> {
        serde_json::from_value(list.to_json())
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

    /// In-place modify to be parseable.
    /// See the comment above for countries for rationale.
    /// Call on root.
    pub fn raise(&mut self) {
        if let Node::List(nodes) = self {
            // Get the first country index
            for (name, tag) in [("provinces", &*PROVINCE_TAG), ("countries", &*COUNTRY_TAG)].iter() {
                if let Some(country_index) = nodes.iter().position(|x| x.is_matching(tag)) {
                    // Drain all countries
                    let country_list: Vec<Node> = nodes.drain_filter(|x| x.is_matching(tag)).collect();
                    nodes.insert(country_index, Node::Line((name, country_list)));
                }
            }
            // log!("{:?}", nodes.iter().filter_map(|x| match x {
            //     Node::Line((txt, _)) | Node::SingleElementLine((txt, _)) => Some(txt),
            //     _ => None,
            // }).fold(String::new(), |s, arg| s + &arg + " "));
        }
    }

    fn is_matching(&self, re: &Regex) -> bool {
        match self {
            Node::Line((name, _)) => re.is_match(name),
            _ => false,
        }
    }

    /// convert a function to serde's json
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
                                        //log!("{}", name);
                                        //unreachable!();
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
pub fn process_save(savetext: &str) -> Result<Save, JsValue> {
    match parse_save(savetext) {
        Ok(entry) => {
            Save::new(entry).map_err(|x| JsValue::from(x.to_string()))
        },
        Err(E) => Err(JsValue::from(E.to_string())),
    }
}
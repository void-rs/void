#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate unicode_segmentation;
extern crate regex;
extern crate time;
extern crate hyper;
extern crate rand;
extern crate termion;
extern crate protobuf;
extern crate libc;

mod logging;
mod serialization;
mod screen;
mod node;
mod pack;
mod meta;
mod plot;
mod task;
mod colors;
mod pb;
mod config;
mod tagdb;
mod dateparse;

use std::cmp;
use std::collections::HashMap;

use regex::Regex;

pub use serialization::{serialize_screen, deserialize_screen};
pub use screen::Screen;
pub use node::Node;
pub use pack::Pack;
pub use colors::random_fg_color;
pub use config::{Config, Action};
pub use logging::init_screen_log;
pub use meta::Meta;
pub use tagdb::TagDB;
pub use dateparse::dateparse;

pub type Coords = (u16, u16);
pub type NodeID = u64;
pub type ScreenDesc = (HashMap<Coords, NodeID>, HashMap<NodeID, Coords>);

#[derive(Debug, PartialEq, Eq)]
pub enum Dir {
    L,
    R,
}

pub fn distances(c1: Coords, c2: Coords) -> (u16, u16) {
    let xcost = cmp::max(c1.0, c2.0) - cmp::min(c1.0, c2.0);
    let ycost = cmp::max(c1.1, c2.1) - cmp::min(c1.1, c2.1);
    (xcost, ycost)
}

pub fn cost(c1: Coords, c2: Coords) -> u16 {
    let (xcost, ycost) = distances(c1, c2);
    xcost + ycost
}

pub fn re_matches<A: std::str::FromStr>(re: &Regex, on: &str) -> Vec<A> {
    let mut ret = vec![];
    if re.is_match(on) {
        for cap in re.captures_iter(on) {
            if let Some(a) = cap.at(1) {
                if let Ok(e) = a.parse::<A>() {
                    ret.push(e)
                }
            }
        }
    }
    ret
}

#[test]
fn test_regex_parsing() {
    let re = Regex::new(r"(\w+)").unwrap();
    assert_eq!(re_matches::<String>(&re, "yo ho ho"),
               vec!["yo".to_owned(), "ho".to_owned(), "ho".to_owned()]);
}

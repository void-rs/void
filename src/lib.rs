#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(
    box_patterns,
    infer_static_outlives_requirements,
    nll,
    slice_patterns,
    test,
    trivial_bounds,
    type_ascription
)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate hyper;
extern crate libc;
extern crate protobuf;
extern crate rand;
extern crate regex;
extern crate termion;
extern crate time;
extern crate unicode_segmentation;

mod colors;
mod config;
mod dateparse;
mod logging;
mod meta;
mod node;
mod pack;
mod pb;
mod plot;
mod screen;
mod serialization;
mod tagdb;
mod task;

use std::{cmp, collections::HashMap};

use regex::Regex;

pub use colors::random_fg_color;
pub use config::{Action, Config};
pub use dateparse::dateparse;
pub use logging::init_screen_log;
pub use crate::meta::Meta;
pub use node::Node;
pub use pack::Pack;
pub use screen::Screen;
pub use serialization::{deserialize_screen, serialize_screen};
pub use tagdb::TagDB;

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
    let re = Regex::new(r"(\S+)").unwrap();
    assert_eq!(
        re_matches::<String>(&re, "yo ho ho"),
        vec!["yo".to_owned(), "ho".to_owned(), "ho".to_owned()]
    );
}

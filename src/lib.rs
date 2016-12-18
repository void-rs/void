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

use std::cmp;
use std::collections::HashMap;

pub use serialization::{serialize_screen, deserialize_screen};
pub use screen::Screen;
pub use node::Node;
pub use pack::Pack;
pub use colors::random_fg_color;
pub use config::{Config, Action};
pub use logging::init_screen_log;
pub use meta::Meta;

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

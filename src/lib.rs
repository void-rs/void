#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate rand;
extern crate quickcheck;
extern crate rustc_serialize;
extern crate bincode;
extern crate termion;
extern crate protobuf;
extern crate rsdb;

mod serialization;
mod logging;
mod screen;
mod node;
mod content;
mod pb;
mod meta;

pub use serialization::{serialize_screen, deserialize_screen};
pub use screen::Screen;
pub use node::Node;
pub use content::Content;
pub use logging::init_screen_log;
pub use meta::Meta;

use std::rc::Rc;
use std::cell::RefCell;

pub type NodeRef = Rc<RefCell<Node>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

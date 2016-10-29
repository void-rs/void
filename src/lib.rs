#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;


extern crate rustc_serialize;
extern crate bincode;
extern crate termion;

mod serialization;
mod logging;
mod screen;
mod node;
mod content;

pub use serialization::{serialize_screen, deserialize_screen};
pub use screen::Screen;
pub use node::Node;
pub use content::Content;
pub use logging::init_screen_log;

// TODO KILL THIS WITH FIRE
pub use serialization::{SerScreen, SerNode};

use std::rc::Rc;
use std::cell::RefCell;
pub type NodeRef = Rc<RefCell<Node>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

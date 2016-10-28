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
mod anchor;
mod screen;
mod node;
mod content;

pub use serialization::{serialize_screen, deserialize_screen};
pub use screen::Screen;
pub use anchor::Anchor;
pub use node::Node;
pub use content::Content;
pub use logging::init_screen_log;

// TODO KILL THIS WITH FIRE
pub use serialization::{SerScreen, SerAnchor, SerNode};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

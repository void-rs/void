mod serialization;
mod screen;
mod node;

pub use self::serialization::{serialize_screen, deserialize_screen};
pub use self::screen::Screen;
pub use self::node::Node;
pub use logging::init_screen_log;
pub use meta::Meta;

use std::rc::Rc;
use std::cell::RefCell;

pub type NodeRef = Rc<RefCell<Node>>;
pub type Coords = (u16, u16);
pub type NodeID = u64;

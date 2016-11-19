mod serialization;
mod screen;
mod node;
mod pack;

use std::cmp;
use std::collections::HashMap;

use rand::{self, Rng};

pub use self::serialization::{serialize_screen, deserialize_screen};
pub use self::screen::Screen;
pub use self::node::Node;
pub use self::pack::Pack;

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

pub fn cost(c1: Coords, c2: Coords) -> u16 {
    let xcost = cmp::max(c1.0, c2.0) - cmp::min(c1.0, c2.0);
    let ycost = cmp::max(c1.1, c2.1) - cmp::min(c1.1, c2.1);
    xcost + ycost
}

pub fn random_color() -> String {
    use termion::color::*;
    let colors: Vec<String> = vec![format!("{}", Fg(LightGreen)),
                                   // format!("{}", Fg(LightBlack)),
                                   format!("{}", Fg(LightRed)),
                                   format!("{}", Fg(LightGreen)),
                                   format!("{}", Fg(LightYellow)),
                                   // format!("{}", Fg(LightBlue)),
                                   format!("{}", Fg(LightMagenta)),
                                   format!("{}", Fg(LightCyan)),
                                   format!("{}", Fg(LightWhite))];
    let c = &*rand::thread_rng().choose(&*colors).unwrap();
    c.clone()
}

trait Mode {}

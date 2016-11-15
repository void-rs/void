mod serialization;
mod screen;
mod node;

use rand::{self, Rng};

pub use self::serialization::{serialize_screen, deserialize_screen};
pub use self::screen::Screen;
pub use self::node::Node;
pub use logging::init_screen_log;
pub use meta::Meta;

pub type Coords = (u16, u16);
pub type NodeID = u64;

pub enum Dir {
    L,
    R,
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

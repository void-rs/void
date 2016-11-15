mod serialization;
mod screen;
mod node;

use std::collections::HashMap;
use std::hash::Hash;

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

// useful for greedy path search
pub struct PrioQueue<A: Hash + Eq, B> {
    inner: HashMap<A, Vec<B>>,
}

impl<A: Hash + Eq, B> Default for PrioQueue<A, B> {
    fn default() -> PrioQueue<A, B> {
        PrioQueue { inner: HashMap::new() }
    }
}

impl<A: Hash + Eq + Clone, B: Clone> PrioQueue<A, B> {
    fn insert(&mut self, k: A, v: B) {
        let mut cur: Vec<B> = self.inner.remove(&k).unwrap_or_else(|| vec![]);
        cur.push(v);
        self.inner.insert(k, cur);
    }
    fn pop(&mut self) -> Option<B> {
        if let Some((lowest_cost, _)) = self.inner.clone().iter().nth(0) {
            let mut cur: Vec<B> = self.inner.remove(lowest_cost).unwrap_or_else(|| vec![]);
            let val = cur.pop();
            if !cur.is_empty() {
                self.inner.insert(lowest_cost.clone(), cur);
            }
            val
        } else {
            None
        }
    }
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

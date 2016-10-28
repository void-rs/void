use std::cell::RefCell;
use std::rc::Rc;

use Node;
// TODO KILL THIS WITH FIRE
use SerAnchor;

#[derive(Debug)]
pub struct Anchor {
    pub head: Rc<RefCell<Node>>,
}

impl Anchor {
    pub fn serialized(&self) -> SerAnchor {
        SerAnchor { head: self.head.borrow().serialized() }
    }

    pub fn draw(&self, x: u16, y: u16) {
        self.head.borrow().draw("".to_string(), x, y, false);
    }

    pub fn lookup(&self, coords: (u16, u16)) -> Option<Rc<RefCell<Node>>> {
        let head = self.head.borrow();
        if coords.1 == 0 {
            if head.content.len() + 1 >= coords.0 as usize {
                Some(self.head.clone())
            } else {
                None
            }
        } else {
            head.lookup(0, coords)
        }
    }

    pub fn height(&self) -> usize {
        self.head.borrow().height()
    }
}

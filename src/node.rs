use std::rc::Rc;
use std::cell::RefCell;

use termion;

use NodeRef;
use Content;
// TODO KILL THIS WITH FIRE
use SerNode;

#[derive(Debug)]
pub struct Node {
    pub content: Content,
    pub children: Vec<NodeRef>,
    pub selected: bool,
    pub collapsed: bool,
}

impl Node {
    pub fn serialized(&self) -> SerNode {
        let ser_children = self.children
            .iter()
            .map(|child| child.borrow().serialized())
            .collect();
        SerNode {
            content: self.content.clone(),
            children: ser_children,
            collapsed: self.collapsed,
        }
    }

    pub fn draw_tree(&self, prefix: String, x: u16, y: u16, last: bool) -> usize {
        print!("{}", termion::cursor::Goto(x, y));

        if self.selected {
            print!("{}", termion::style::Invert);
        }

        if prefix == "" {
            print!("⚒ ");
        }

        print!("{}", prefix);

        if prefix != "" {
            if last {
                print!("└─ ");
            } else {
                print!("├─ ");
            }
        }

        self.content.draw();

        if self.collapsed {
            print!("…");
        }

        if self.selected {
            print!("{}", termion::style::Reset);
        }

        println!("");

        let mut drawn = 1;
        let mut prefix = prefix;
        if last {
            prefix.push_str("   ");
        } else if prefix == "" {
            prefix.push_str("  ");
        } else {
            prefix.push_str("│  ");
        }
        if !self.collapsed {
            let n_children = self.children.len();
            for (n, child) in self.children.iter().enumerate() {
                let last = n + 1 == n_children;
                drawn += child.borrow().draw_tree(prefix.clone(), x, y + drawn as u16, last);
            }
        }

        drawn
    }

    pub fn find_child_at_coords(&self, depth: usize, coords: (u16, u16)) -> Option<NodeRef> {
        let mut y_traversed = 1;
        for child in &self.children {
            if coords.1 == y_traversed {
                if child.borrow().content.len() + 1 + (3 * (depth + 1)) >= coords.0 as usize {
                    return Some(child.clone());
                } else {
                    return None;
                }
            } else if coords.1 < y_traversed + child.borrow().height() as u16 {
                return child.borrow()
                    .find_child_at_coords(depth + 1, (coords.0, coords.1 - y_traversed));
            } else {
                y_traversed += child.borrow().height() as u16;
            }
        }

        None
    }

    pub fn height(&self) -> usize {
        if self.collapsed {
            1
        } else {
            self.children.iter().fold(1, |acc, c| acc + c.borrow().height())
        }
    }

    pub fn delete(&mut self, node: NodeRef) -> bool {
        let ptr = {
            node.as_ptr()
        };
        let mut contains = false;
        for child in &self.children {
            if ptr == child.as_ptr() {
                info!("deleting child {:?}", node.borrow().content);
                contains = true;
            }
        }
        if contains {
            let children = self.children.clone();
            let new_children = children.into_iter().filter(|c| ptr != c.as_ptr()).collect();
            self.children = new_children;
            return true;
        }
        self.children.iter().fold(false, |acc, c| {
            if acc {
                true
            } else {
                c.borrow_mut().delete(node.clone())
            }
        })
    }

    pub fn toggle_collapsed(&mut self) {
        if self.collapsed {
            self.collapsed = false;
        } else {
            self.collapsed = true;
        }
    }

    pub fn create_child(&mut self) -> NodeRef {
        let new = node("", vec![]);
        let child = Rc::new(RefCell::new(new));
        self.children.push(child.clone());
        child
    }

    pub fn flat_children(&self) -> Vec<NodeRef> {
        self.children
            .iter()
            .fold(vec![], |mut acc, child| {
                acc.push(child.clone());
                acc.extend(child.borrow().flat_children());
                acc
            })
    }
}

fn node(text: &str, children: Vec<Node>) -> Node {
    let rc_children = children.into_iter().map(|child| Rc::new(RefCell::new(child))).collect();

    Node {
        content: Content::Text { text: text.to_string() },
        children: rc_children,
        selected: false,
        collapsed: false,
    }
}

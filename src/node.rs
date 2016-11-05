use std::rc::Rc;
use std::cell::RefCell;

use termion;

use {NodeRef, Content, Meta};

#[derive(Debug)]
pub struct Node {
    pub content: Content,
    pub children: Vec<NodeRef>,
    pub selected: bool,
    pub collapsed: bool,
    pub stricken: bool,
    pub hide_stricken: bool,
    pub meta: Meta,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            content: Content::Text { text: "".to_string() },
            children: vec![],
            selected: false,
            collapsed: false,
            stricken: false,
            hide_stricken: false,
            meta: Meta::default(),
        }
    }
}

impl Node {
    pub fn draw_tree(&self, prefix: String, x: u16, y: u16, last: bool) -> usize {
        print!("{}", termion::cursor::Goto(x, y));

        if self.selected {
            print!("{}", termion::style::Invert);
        }

        print!("{}", prefix);

        if prefix != "" {
            // only anchor will have blank prefix
            if last {
                print!("└─");
            } else {
                print!("├─");
            }
        }

        if self.stricken {
            print!("☠");
        } else if prefix == "" {
            print!("⚒");
        } else {
            print!(" ");
        }

        if prefix == "" {
            print!(" ");
        }

        self.content.draw();

        if self.collapsed {
            print!("…");
        }

        println!("{}", termion::style::Reset);

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

    pub fn find_child_at_coords(&self,
                                depth: usize,
                                coords: (u16, u16))
                                -> Result<NodeRef, String> {
        let mut y_traversed = 1;
        for child in &self.children {
            if coords.1 == y_traversed {
                if child.borrow().content.len() + 1 + (3 * (depth + 1)) >= coords.0 as usize {
                    return Ok(child.clone());
                } else {
                    return Err("could not find node at this location".to_string());
                }
            } else if coords.1 < y_traversed + child.borrow().height() as u16 {
                return child.borrow()
                    .find_child_at_coords(depth + 1, (coords.0, coords.1 - y_traversed));
            } else {
                y_traversed += child.borrow().height() as u16;
            }
        }

        Err("could not find node at this location".to_string())
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
        let new = Node::default();
        let child = Rc::new(RefCell::new(new));
        self.children.push(child.clone());
        child
    }

    pub fn flat_visible_children(&self) -> Vec<NodeRef> {
        self.children
            .iter()
            .fold(vec![], |mut acc, child| {
                if !self.collapsed {
                    acc.push(child.clone());
                    acc.extend(child.borrow().flat_visible_children());
                }
                acc
            })
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

    pub fn toggle_stricken(&mut self) {
        if self.stricken {
            self.stricken = false;
        } else {
            self.stricken = true;
        }
    }

    // TODO make these toggle things macros
    pub fn toggle_hide_stricken(&mut self) {
        if self.hide_stricken {
            self.hide_stricken = false;
        } else {
            self.hide_stricken = true;
        }
    }
}

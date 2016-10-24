extern crate climate;
extern crate termion;

use std::collections::BTreeMap;
use std::io::{Write, Stdout, stdout, stdin};
use std::cell::RefCell;
use std::ops::{Div, Mul};
use std::rc::Rc;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

struct Screen {
    anchors: BTreeMap<(u16, u16), Rc<RefCell<Anchor>>>,
    last_selected: Option<(Rc<RefCell<Anchor>>, Rc<RefCell<Node>>)>,
    stdout: Option<MouseTerminal<RawTerminal<Stdout>>>,
    logs: RefCell<Vec<String>>,
}

impl Screen {
    fn draw(&mut self) {
        // clear screen
        print!("\x1b[2J\x1b[H");

        for (coords, anchor) in self.anchors.iter() {
            anchor.borrow().draw(coords.0, coords.1);
        }

        let (_, bottom) = termion::terminal_size().unwrap();
        println!("{}logs:", termion::cursor::Goto(0, bottom - 11));
        {
            let logs = self.logs.borrow();
            for msg in logs.iter().rev() {
                println!("\r{}", msg);
            }
        }
        let mut s = self.stdout.take().unwrap();
        s.flush().unwrap();
        self.stdout = Some(s);
    }

    fn insert(&mut self, coords: (u16, u16), anchor: Anchor) {
        self.anchors.insert(coords, Rc::new(RefCell::new(anchor)));
    }

    fn log<'a>(&self, msg: &'a str) {
        let mut logs = self.logs.borrow_mut();
        logs.insert(0, msg.to_string());
        logs.truncate(10);
    }

    fn lookup(&mut self, coords: (u16, u16)) -> Option<(Rc<RefCell<Anchor>>, Rc<RefCell<Node>>)> {
        // scan possible anchors
        let mut candidate_anchors = vec![];
        for (&(x, y), anchor) in self.anchors.iter() {
            if coords.0 >= x && coords.1 >= y && coords.1 - y < anchor.borrow().children() as u16 {
                candidate_anchors.push(((x, y), anchor.clone()));
            }
        }
        // scan possible nodes
        let mut candidate_nodes = vec![];
        for ((x, y), anchor) in candidate_anchors {
            if let Some(node) = anchor.borrow().lookup((coords.0 - x, coords.1 - y)) {
                candidate_nodes.push((anchor.clone(), node));
            }
        }
        self.log(&format!("found {} matching nodes", candidate_nodes.len()));
        candidate_nodes.pop()
    }

    fn try_select(&mut self, x: u16, y: u16) {
        if let Some((_, ref old_node)) = self.last_selected {
            old_node.borrow_mut().selected = false;
        }
        if let Some((anchor, node)) = self.lookup((x, y)) {
            node.borrow_mut().selected = true;
            self.last_selected = Some((anchor, node.clone()))
        }
    }

    fn delete_selected(&mut self) {
        if let Some((ref anchor, ref node)) = self.last_selected {
            if anchor.borrow().head.as_ptr() == node.as_ptr() {
                // nuke whole anchor
            } else {
                anchor.borrow_mut().delete(node.clone());
            }
        }
    }

    fn expand_selected(&mut self) {}

    fn run(&mut self) {
        if self.stdout.is_none() {
            self.stdout = Some(MouseTerminal::from(stdout().into_raw_mode().unwrap()));
        }
        self.draw();
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char('q')) => break,
                Event::Key(Key::Char('\t')) => self.expand_selected(),
                Event::Key(Key::Delete) => self.delete_selected(),
                Event::Mouse(me) => {
                    match me {
                        MouseEvent::Press(_, x, y) => {
                            self.try_select(x, y);
                        }
                        MouseEvent::Release(x, y) => {}
                        e => self.log(&format!("Weird mouse event {:?}", e)),
                    }
                }
                e => self.log(&format!("Weird event {:?}", e)),
            }
            self.draw();
        }
    }
}

impl Default for Screen {
    fn default() -> Screen {
        Screen {
            anchors: BTreeMap::new(),
            last_selected: None,
            stdout: None,
            logs: RefCell::new(vec![]),
        }
    }
}

struct Anchor {
    head: Rc<RefCell<Node>>,
}

impl Anchor {
    fn draw(&self, x: u16, y: u16) {
        self.head.borrow().draw(0, x, y, false);
    }
    fn lookup(&self, coords: (u16, u16)) -> Option<Rc<RefCell<Node>>> {
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

    fn delete(&mut self, node: Rc<RefCell<Node>>) -> bool {
        false
    }

    fn children(&self) -> usize {
        self.head.borrow().children()
    }
}

#[derive(Debug)]
enum Content {
    Text(String),
    Plot(Vec<i64>),
}

impl Content {
    fn draw(&self) {
        match self {
            &Content::Text(ref text) => println!("{}", text),
            &Content::Plot(ref data) => plot_graph(data.clone()),
        }
    }
    fn len(&self) -> usize {
        match self {
            &Content::Text(ref text) => text.len(),
            &Content::Plot(ref data) => data.len(),
        }
    }
}

#[derive(Debug)]
struct Node {
    content: Content,
    children: Vec<Rc<RefCell<Node>>>,
    selected: bool,
}

impl Node {
    fn draw(&self, depth: usize, x: u16, y: u16, last: bool) -> usize {
        let mut drawn = 1;
        print!("{}", termion::cursor::Goto(x, y));

        if self.selected {
            print!("{}", termion::style::Invert);
        }

        if depth == 0 {
            print!("⚒ ");
        } else {
            print!("  ");
        }

        for _ in 1..depth {
            print!("│  ");
        }

        if depth != 0 {
            if last {
                print!("└─ ");
            } else {
                print!("├─ ");
            }
        }

        self.content.draw();

        if self.selected {
            print!("{}", termion::style::Reset);
        }

        let n_children = self.children.len();
        for (n, child) in self.children.iter().enumerate() {
            let last = if n + 1 == n_children {
                true
            } else {
                false
            };

            drawn += child.borrow().draw(depth + 1, x, y + drawn as u16, last);
        }

        drawn
    }
    fn lookup(&self, depth: usize, coords: (u16, u16)) -> Option<Rc<RefCell<Node>>> {
        let mut y_traversed = 1;
        for child in self.children.iter() {
            if coords.1 == y_traversed {
                if child.borrow().content.len() + 1 + (3 * (depth + 1)) >= coords.0 as usize {
                    return Some(child.clone());
                } else {
                    return None;
                }
            } else if coords.1 < y_traversed + child.borrow().children() as u16 {
                return child.borrow().lookup(depth + 1, (coords.0, coords.1 - y_traversed));
            } else {
                y_traversed += child.borrow().children() as u16;
            }
        }

        None
    }
    fn children(&self) -> usize {
        self.children.iter().fold(1, |acc, c| acc + c.borrow().children())
    }
}

fn node(text: &str, children: Vec<Node>) -> Node {
    let rc_children = children.into_iter().map(|child| Rc::new(RefCell::new(child))).collect();

    Node {
        content: Content::Text(text.to_string()),
        children: rc_children,
        selected: false,
    }
}

fn plot_graph<T>(nums_in: Vec<T>)
    where T: Into<i64>
{
    const bars: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let nums: Vec<_> = nums_in.into_iter().map(|n| n.into()).collect();
    let max = nums.iter().max();

    for n in nums.iter() {
        let idx = (bars.len() - 1) as i64 * n / max.unwrap();
        print!("{}", bars[idx as usize]);
    }
}

fn main() {
    let other = node("other", vec![]);
    let next = node("next", vec![]);
    let zone = node("zone", vec![]);
    let plot = Node {
        content: Content::Plot(vec![1, 2, 5, 2, 3]),
        children: vec![],
        selected: false,
    };
    let bone = node("bone", vec![plot]);
    let one = node("one", vec![bone, zone]);
    let header = node("header", vec![one, next, other]);

    let mut anchor = Anchor { head: Rc::new(RefCell::new(header)) };

    let mut scene = Screen::default();
    scene.insert((3, 4), anchor);
    plot_graph(vec![1, 2, 3, 4]);
    scene.run();
}

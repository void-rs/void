extern crate climate;
extern crate termion;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

struct Anchor {
    x: u16,
    y: u16,
    head: Node,
}

impl Anchor {
    fn draw(&self) {
        self.head.draw(0, self.x, self.y, false);
    }
}

struct Node {
    text: String,
    children: Vec<Node>,
}

impl Node {
    fn draw(&self, depth: usize, x: u16, y: u16, last: bool) -> usize {
        let mut drawn = 1;
        print!("{}", termion::cursor::Goto(x, y));

        if depth == 0 {
            print!("⚒ ");
        } else {
            print!("  ");
        }

        for i in 1..depth {
            print!("│  ");
        }

        if depth != 0 {
            if last {
                print!("└─ ");
            } else {
                print!("├─ ");
            }
        }

        println!("{}", self.text);
        // std::thread::sleep_ms(60);

        let n_children = self.children.len();
        for (n, child) in self.children.iter().enumerate() {
            let last = if n + 1 == n_children {
                true
            } else {
                false
            };

            drawn += child.draw(depth + 1, x, y + drawn as u16, last);
        }

        drawn
    }
}

fn node(text: &str, children: Vec<Node>) -> Node {
    Node {
        text: text.to_string(),
        children: children,
    }
}

fn clear() {
    print!("\x1b[2J\x1b[H");
}

fn main() {
    clear();
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());


    let other = node("other", vec![]);
    let next = node("next", vec![]);
    let zone = node("zone", vec![]);
    let bone = node("bone", vec![]);
    let one = node("one", vec![bone, zone]);
    let header = node("header", vec![one, next, other]);
    let mut anchor = Anchor {
        x: 3,
        y: 4,
        head: header,
    };
    anchor.draw();

    // println!("");
    //
    // println!("header");
    // println!("├─ one");
    // println!("│  ├─ bone");
    // println!("│  └─ zone");
    // println!("├─ next");
    // println!("└─ other");
    //

    let mut dragging = false;

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        if !dragging && x == anchor.x && y == anchor.y {
                            dragging = true;
                        }
                        // write!(stdout, "{}{},{}", termion::cursor::Goto(x, y), x, y).unwrap();
                    }
                    MouseEvent::Release(x, y) => {
                        if dragging {
                            dragging = false;
                            anchor.x = x;
                            anchor.y = y;
                            clear();
                            anchor.draw();
                        }
                    }
                    e => println!("{:?}", e),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

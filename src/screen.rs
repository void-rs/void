use std::fs::{File, rename, remove_file};
use std::collections::BTreeMap;
use std::io::{Write, Stdout, stdout, stdin};
use std::cell::RefCell;
use std::rc::Rc;
use std::process::exit;

use termion;
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

use {Node, Content};
// TODO KILL THIS WITH FIRE
use SerScreen;

use serialization;
use logging;

// anchor node, selected node
type Lookup = Option<(Rc<RefCell<Node>>, Rc<RefCell<Node>>)>;

pub struct Screen {
    pub anchors: BTreeMap<(u16, u16), Rc<RefCell<Node>>>,
    pub last_selected: Lookup,
    stdout: Option<MouseTerminal<RawTerminal<Stdout>>>,
    dragging_from: Option<(u16, u16)>,
    pub work_path: Option<String>,
}

impl Default for Screen {
    fn default() -> Screen {
        Screen {
            anchors: BTreeMap::new(),
            last_selected: None,
            stdout: None,
            dragging_from: None,
            work_path: None,
        }
    }
}

impl Screen {
    pub fn serialized(&self) -> SerScreen {
        let mut ser_anchors = BTreeMap::new();
        for (coords, anchor) in &self.anchors {
            ser_anchors.insert(*coords, anchor.borrow().serialized());
        }
        SerScreen { anchors: ser_anchors }
    }

    fn draw(&mut self) {
        // clear screen
        print!("\x1b[2J\x1b[H");

        for (coords, anchor) in &self.anchors {
            anchor.borrow().draw_tree("".to_string(), coords.0, coords.1, false);
        }

        // print logs
        let (width, bottom) = termion::terminal_size().unwrap();
        if width > 4 && bottom > 7 {
            let mut sep = format!("{}{}logs{}",
                                  termion::cursor::Goto(0, bottom - 6),
                                  termion::style::Invert,
                                  termion::style::Reset);
            for _ in 0..width - 4 {
                sep.push('█');
            }
            println!("{}{}", termion::cursor::Goto(0, bottom - 12), sep);
            {
                let logs = logging::read_logs();
                for msg in logs.iter().rev() {
                    println!("\r{}", msg);
                }
            }
        }

        print!("{}", termion::cursor::Hide);
        let mut s = self.stdout.take().unwrap();
        s.flush().unwrap();
        self.stdout = Some(s);
    }

    fn insert(&mut self, coords: (u16, u16), node: Node) {
        self.anchors.insert(coords, Rc::new(RefCell::new(node)));
    }

    fn lookup(&mut self, coords: (u16, u16)) -> Lookup {
        // scan possible anchors
        let mut candidate_anchors = vec![];
        for (&(x, y), anchor) in &self.anchors {
            if coords.0 >= x && coords.1 >= y && coords.1 - y < anchor.borrow().height() as u16 {
                candidate_anchors.push(((x, y), anchor.clone()));
            }
        }
        // scan possible nodes
        let mut candidate_nodes = vec![];
        for ((x, y), anchor) in candidate_anchors {
            let lookup_coords = (coords.0 - x, coords.1 - y);
            let look = if lookup_coords.1 == 0 {
                if anchor.borrow().content.len() + 1 >= lookup_coords.0 as usize {
                    Some(anchor.clone())
                } else {
                    None
                }
            } else {
                anchor.borrow().lookup(0, lookup_coords)
            };
            if let Some(node) = look {
                candidate_nodes.push((anchor.clone(), node));
            }
        }
        candidate_nodes.pop()
    }

    fn pop_selected(&mut self) -> Lookup {
        if self.dragging_from.is_none() {
            if let Some((anchor, old_node)) = self.last_selected.take() {
                old_node.borrow_mut().selected = false;
                Some((anchor, old_node))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn try_select(&mut self, x: u16, y: u16) -> Lookup {
        if self.dragging_from.is_none() {
            if let Some((anchor, node)) = self.lookup((x, y)) {
                node.borrow_mut().selected = true;
                self.last_selected = Some((anchor.clone(), node.clone()));
                self.dragging_from = Some((x, y));
                Some((anchor, node))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn delete_selected(&mut self) {
        if let Some((ref anchor, ref node)) = self.last_selected {
            let ptr = {
                anchor.as_ptr()
            };
            if ptr == node.as_ptr() {
                info!("deleting anchor {:?}", node.borrow().content);
                // nuke whole anchor
                let anchors = self.anchors
                    .clone()
                    .into_iter()
                    .filter(|&(_, ref anchor)| anchor.as_ptr() != ptr)
                    .collect();
                self.anchors = anchors;
            } else {
                anchor.borrow_mut().delete(node.clone());
            }
        }
    }

    fn create_child(&mut self) {
        if let Some((_, ref selected)) = self.last_selected {
            selected.borrow_mut().create_child()
        }
    }

    pub fn run(&mut self) {
        if self.stdout.is_none() {
            self.stdout = Some(MouseTerminal::from(stdout().into_raw_mode().unwrap()));
        }
        self.draw();
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();
            self.handle_event(evt);
            self.draw();
        }
    }

    fn toggle_collapsed(&mut self) {
        if let Some((_, ref selected)) = self.last_selected {
            selected.borrow_mut().toggle_collapsed()
        }
    }

    fn create_anchor(&mut self, coords: (u16, u16)) {
        let node = Node {
            content: Content::Text { text: "".to_string() },
            children: vec![],
            selected: false,
            collapsed: false,
        };
        self.insert(coords, node);
    }

    fn backspace(&mut self) {
        if let Some((_, ref selected)) = self.last_selected {
            let mut node = selected.borrow_mut();
            node.content.backspace();
        }
    }

    fn append(&mut self, c: char) {
        if let Some((_, ref selected)) = self.last_selected {
            let mut node = selected.borrow_mut();
            node.content.append(c);
        }
    }

    fn move_selected(&mut self, from: (u16, u16), to: (u16, u16)) {
        let dx = to.0 as i16 - from.0 as i16;
        let dy = to.1 as i16 - from.1 as i16;

        let anchors_clone = self.anchors.clone();
        if let Some((ref anchor, _)) = self.last_selected {
            for (coords, value) in &anchors_clone {
                let nx = (coords.0 as i16 + dx) as u16;
                let ny = (coords.1 as i16 + dy) as u16;
                if value.as_ptr() == anchor.as_ptr() {
                    let anchor = self.anchors.remove(coords).unwrap();
                    self.anchors.insert((nx, ny), anchor);
                }
            }
        }
    }

    fn handle_event(&mut self, evt: Event) {
        match evt {
            Event::Key(Key::Char('\n')) => self.toggle_collapsed(),
            Event::Key(Key::Char('\t')) => self.create_child(),
            Event::Key(Key::Delete) => self.delete_selected(),
            Event::Key(Key::Alt('\u{1b}')) |
            Event::Key(Key::Ctrl('c')) |
            Event::Key(Key::Ctrl('d')) => self.exit(),
            Event::Key(Key::Ctrl('w')) => self.save(),
            Event::Key(Key::Backspace) => self.backspace(),
            Event::Key(Key::Char(c)) => self.append(c),
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        let old = self.pop_selected();
                        self.try_select(x, y);
                        if old.is_none() && self.dragging_from.is_none() {
                            self.create_anchor((x, y));
                        }
                    }
                    MouseEvent::Release(x, y) => {
                        if let Some((from_x, from_y)) = self.dragging_from.take() {
                            self.move_selected((from_x, from_y), (x, y));
                        }
                    }
                    MouseEvent::Hold(..) => {
                        // this isn't supported in some terminals
                        // (urxvt...) so don't rely on it
                    }
                }
            }
            e => warn!("Weird event {:?}", e),
        }
    }

    fn save(&self) {
        let data = serialization::serialize_screen(self);
        if let Some(ref path) = self.work_path {
            let mut tmp_path = path.clone();
            tmp_path.push_str(".tmp");
            remove_file(&tmp_path);
            let mut f = File::create(&tmp_path).unwrap();
            f.write_all(&*data).unwrap();
            rename(tmp_path, path).unwrap();
            info!("saved work to {}", path);
        }
    }

    fn exit(&mut self) {
        let (_, bottom) = termion::terminal_size().unwrap();
        print!("{}", termion::cursor::Goto(0, bottom));
        println!("{}", termion::cursor::Show);
        self.stdout.take().unwrap().flush().unwrap();
        self.save();
        exit(0);
    }
}

#![feature(plugin)]
#![plugin(clippy)]
extern crate getopts;
extern crate bincode;
extern crate rustc_serialize;
extern crate climate;
extern crate termion;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use std::fs::{File, rename, remove_file};
use std::collections::BTreeMap;
use std::io::{Write, Stdout, stdout, stdin, Read, Error, ErrorKind};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};
use getopts::Options;
use log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata, SetLoggerError};
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

struct ScreenLogger;

impl log::Log for ScreenLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let line = format!("{} - {}", record.level(), record.args());
            let mut logs = LOGS.write().unwrap();
            logs.insert(0, line);
            logs.truncate(10);
        }
    }
}

pub fn init_screen_log() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Debug);
        Box::new(ScreenLogger)
    })
}

lazy_static! {
    static ref LOGS: RwLock<Vec<String>> = RwLock::new(vec![]);
}

type Lookup = Option<(Rc<RefCell<Anchor>>, Rc<RefCell<Node>>)>;

// #[derive(Debug)]
struct Screen {
    anchors: BTreeMap<(u16, u16), Rc<RefCell<Anchor>>>,
    last_selected: Lookup,
    stdout: Option<MouseTerminal<RawTerminal<Stdout>>>,
    dragging_from: Option<(u16, u16)>,
    work_path: Option<String>,
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
    fn serialized(&self) -> SerScreen {
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
            anchor.borrow().draw(coords.0, coords.1);
        }

        // print logs
        let (width, bottom) = termion::terminal_size().unwrap();
        if width > 4 && bottom > 12 {
            let mut sep = format!("{}{}logs{}",
                                  termion::cursor::Goto(0, bottom - 11),
                                  termion::style::Invert,
                                  termion::style::Reset);
            for _ in 0..width - 4 {
                sep.push('█');
            }
            println!("{}{}", termion::cursor::Goto(0, bottom - 12), sep);
            {
                let logs = LOGS.read().unwrap();
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

    fn insert(&mut self, coords: (u16, u16), anchor: Anchor) {
        self.anchors.insert(coords, Rc::new(RefCell::new(anchor)));
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
            if let Some(node) = anchor.borrow().lookup((coords.0 - x, coords.1 - y)) {
                candidate_nodes.push((anchor.clone(), node));
            }
        }
        candidate_nodes.pop()
    }

    // down on selectable
    //      1. try to select
    //      1. drag = true
    // up on selectable
    //      1. drag = false
    // down on nothing
    //      1. drag selected
    //      1. deselect
    // up on nothing
    //      1. move if selected
    //      1. drag = false
    fn try_select(&mut self, x: u16, y: u16) {
        if self.dragging_from.is_none() {
            if let Some((_, old_node)) = self.last_selected.take() {
                old_node.borrow_mut().selected = false;
            }
            if let Some((anchor, node)) = self.lookup((x, y)) {
                node.borrow_mut().selected = true;
                self.last_selected = Some((anchor, node.clone()));
                self.dragging_from = Some((x, y));
            }
        }
    }

    fn delete_selected(&mut self) {
        if let Some((ref anchor, ref node)) = self.last_selected {
            let ptr = {
                anchor.borrow().head.as_ptr()
            };
            if ptr == node.as_ptr() {
                info!("deleting anchor {:?}", node.borrow().content);
                // nuke whole anchor
                let anchors = self.anchors
                    .clone()
                    .into_iter()
                    .filter(|&(_, ref anchor)| anchor.borrow().head.as_ptr() != ptr)
                    .collect();
                self.anchors = anchors;
            } else {
                let anchor = anchor.borrow();
                anchor.head.borrow_mut().delete(node.clone());
            }
        }
    }

    fn create_child(&mut self) {
        if let Some((_, ref selected)) = self.last_selected {
            selected.borrow_mut().create_child()
        }
    }

    fn run(&mut self) {
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
        let header = node("", vec![]);
        let anchor = Anchor { head: Rc::new(RefCell::new(header)) };
        self.insert(coords, anchor);
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
                        self.try_select(x, y);
                    }
                    MouseEvent::Release(x, y) => {
                        if let Some((from_x, from_y)) = self.dragging_from.take() {
                            self.move_selected((from_x, from_y), (x, y));
                        } else if self.last_selected.is_none() {
                            self.create_anchor((x, y));
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
        let serialized_self = self.serialized();
        let data = encode(&serialized_self, SizeLimit::Infinite).unwrap();
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
        std::process::exit(0);
    }
}

#[derive(Debug)]
struct Anchor {
    head: Rc<RefCell<Node>>,
}

impl Anchor {
    fn serialized(&self) -> SerAnchor {
        SerAnchor { head: self.head.borrow().serialized() }
    }

    fn draw(&self, x: u16, y: u16) {
        self.head.borrow().draw("".to_string(), x, y, false);
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

    fn height(&self) -> usize {
        self.head.borrow().height()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
struct SerScreen {
    anchors: BTreeMap<(u16, u16), SerAnchor>,
}

impl SerScreen {
    fn deserialize(&self) -> Screen {
        let mut screen = Screen::default();
        let mut anchors = BTreeMap::new();
        for (coords, anchor) in &self.anchors {
            anchors.insert(*coords, anchor.deserialize());
        }
        screen.anchors = anchors;
        screen
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
struct SerNode {
    content: Content,
    children: Vec<SerNode>,
}

impl SerNode {
    fn deserialize(&self) -> Rc<RefCell<Node>> {
        let children = self.children
            .iter()
            .map(|ser_child| ser_child.deserialize())
            .collect();
        Rc::new(RefCell::new(Node {
            content: self.content.clone(),
            children: children,
            selected: false,
            collapsed: false,
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
struct SerAnchor {
    head: SerNode,
}

impl SerAnchor {
    fn deserialize(&self) -> Rc<RefCell<Anchor>> {
        Rc::new(RefCell::new(Anchor { head: self.head.deserialize() }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
enum Content {
    Text {
        text: String,
    },
    Plot(Vec<i64>),
}

impl Content {
    fn draw(&self) {
        match *self {
            Content::Text { ref text } => print!("{}", text),
            Content::Plot(ref data) => plot_graph(data.clone()),
        }
    }
    fn len(&self) -> usize {
        match *self {
            Content::Text { ref text } => text.len(),
            Content::Plot(ref data) => data.len(),
        }
    }
    fn backspace(&mut self) {
        match *self {
            Content::Text { ref mut text } => {
                let newlen = std::cmp::max(text.len(), 1) - 1;
                *text = text.clone()[..newlen].to_string();
            }
            Content::Plot(_) => unimplemented!(),
        }
    }
    fn append(&mut self, c: char) {
        match *self {
            Content::Text { ref mut text } => {
                text.push(c);
            }
            Content::Plot(_) => {
                unimplemented!();
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    content: Content,
    children: Vec<Rc<RefCell<Node>>>,
    selected: bool,
    collapsed: bool,
}

impl Node {
    fn serialized(&self) -> SerNode {
        let ser_children = self.children
            .iter()
            .map(|child| child.borrow().serialized())
            .collect();
        SerNode {
            content: self.content.clone(),
            children: ser_children,
        }
    }


    fn draw(&self, prefix: String, x: u16, y: u16, last: bool) -> usize {
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
                drawn += child.borrow().draw(prefix.clone(), x, y + drawn as u16, last);
            }
        }

        drawn
    }
    fn lookup(&self, depth: usize, coords: (u16, u16)) -> Option<Rc<RefCell<Node>>> {
        let mut y_traversed = 1;
        for child in &self.children {
            if coords.1 == y_traversed {
                if child.borrow().content.len() + 1 + (3 * (depth + 1)) >= coords.0 as usize {
                    return Some(child.clone());
                } else {
                    return None;
                }
            } else if coords.1 < y_traversed + child.borrow().height() as u16 {
                return child.borrow().lookup(depth + 1, (coords.0, coords.1 - y_traversed));
            } else {
                y_traversed += child.borrow().height() as u16;
            }
        }

        None
    }

    fn height(&self) -> usize {
        if self.collapsed {
            1
        } else {
            self.children.iter().fold(1, |acc, c| acc + c.borrow().height())
        }
    }

    fn delete(&mut self, node: Rc<RefCell<Node>>) -> bool {
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
    fn toggle_collapsed(&mut self) {
        if self.collapsed {
            self.collapsed = false;
        } else {
            self.collapsed = true;
        }
    }
    fn create_child(&mut self) {
        let new = node("", vec![]);
        self.children.push(Rc::new(RefCell::new(new)));
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

fn plot_graph<T>(nums_in: Vec<T>)
    where T: Into<i64>
{
    const BARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let nums: Vec<_> = nums_in.into_iter().map(|n| n.into()).collect();
    let max = nums.iter().max();

    for n in &nums {
        let idx = (BARS.len() - 1) as i64 * n / max.unwrap();
        print!("{}", BARS[idx as usize]);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <--file=/path/to/workfile>", program);
    print!("{}", opts.usage(&brief));
}

fn parse_workfile_path() -> std::io::Result<String> {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "file", "use a workfile", "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    matches.opt_str("f").ok_or_else(|| Error::new(ErrorKind::Other, "no path provided"))
}

fn main() {
    init_screen_log().unwrap();

    // load from file if present
    let work_data = parse_workfile_path().and_then(|path| {
        let mut data = vec![];
        let mut f = try!(File::open(path));
        f.read_to_end(&mut data).unwrap();
        Ok(data)
    });

    let saved_screen: std::io::Result<Screen> = work_data.map(|data| {
        let ser_screen: SerScreen = decode(&data[..]).unwrap();
        info!("loaded saved data");
        ser_screen.deserialize()
    });

    let plot = Node {
        content: Content::Plot(vec![1, 2, 5, 2, 3]),
        children: vec![],
        selected: false,
        collapsed: false,
    };
    let task_nodes = vec![
        node("[ ] get location fn", vec![]),
        node("[ ] states", vec![]),
    ];
    let tasks = node("tasks", task_nodes);
    let tracking_nodes = vec![
        node("[ ] metadata", vec![]),
        node("[ ] edit store", vec![]),
        node("[ ] history", vec![]),
    ];
    let tracking = node("tracking", tracking_nodes);
    let vizualization_nodes = vec![
        node("[ ] plot graph for tags, time", vec![plot]),
    ];
    let vizualization = node("vizualization", vizualization_nodes);
    let serialization_nodes = vec![
        node("[ ] rustc ser/de", vec![]),
        node("[ ] log", vec![]),
    ];
    let serialization = node("serialization", serialization_nodes);

    let todo = node("todo", vec![tasks, tracking, vizualization, serialization]);
    let anchor = Anchor { head: Rc::new(RefCell::new(todo)) };

    let mut def_screen = Screen::default();
    def_screen.insert((3, 4), anchor);

    let mut screen = saved_screen.unwrap_or(def_screen);
    screen.work_path = parse_workfile_path().ok();
    screen.run();
}

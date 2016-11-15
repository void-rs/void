use std;
use std::env;
use std::cmp;
use std::fs::{File, rename, remove_file, OpenOptions};
use std::collections::{BTreeMap, HashMap};
use std::process;
use std::io::{Write, Read, Seek, SeekFrom, Stdout, stdout, stdin};

use termion::{terminal_size, cursor, style, clear};
use termion::color;
use termion::event::{Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

use time;

use rand;
use rand::distributions::{IndependentSample, Range};

use plot::plot_sparkline;
use mindmap::{NodeID, Coords, Node, serialization, random_color, PrioQueue, Dir};
use logging;

pub struct Screen {
    pub max_id: u64,
    pub drawing_root: NodeID,
    pub nodes: HashMap<NodeID, Node>,
    pub arrows: Vec<(NodeID, NodeID)>,
    pub work_path: Option<String>,
    pub show_logs: bool,
    pub show_meta: bool,
    last_selected: Option<NodeID>,
    drawing_arrow: Option<NodeID>,
    dragging_from: Option<Coords>,
    lookup: HashMap<Coords, NodeID>,
    drawn_at: HashMap<NodeID, Coords>,
    stdout: Option<MouseTerminal<RawTerminal<Stdout>>>,
}

impl Default for Screen {
    fn default() -> Screen {
        let mut root = Node::default();
        root.content = "home".to_owned();
        let mut screen = Screen {
            arrows: vec![],
            last_selected: None,
            drawing_arrow: None,
            nodes: HashMap::new(),
            lookup: HashMap::new(),
            drawn_at: HashMap::new(),
            show_logs: false,
            show_meta: true,
            drawing_root: 0,
            stdout: None,
            dragging_from: None,
            work_path: None,
            max_id: 0,
        };
        screen.nodes.insert(0, root);
        screen
    }
}

impl Screen {
    fn new_node(&mut self) -> NodeID {
        let mut node = Node::default();
        self.max_id += 1;
        let id = self.max_id;
        node.id = id;
        self.nodes.insert(id, node);
        id
    }

    fn with_node<B, F>(&self, k: NodeID, mut f: F) -> Option<B>
        where F: FnMut(&Node) -> B
    {
        self.nodes.get(&k).map(|node| f(node))
    }

    fn with_node_mut<B, F>(&mut self, k: NodeID, mut f: F) -> Option<B>
        where F: FnMut(&mut Node) -> B
    {
        self.nodes.get_mut(&k).map(|mut node| {
            node.meta.bump_mtime();
            f(&mut node)
        })
    }

    // return of false signals to the caller that we are done in this view
    fn handle_event(&mut self, evt: Event) -> bool {
        use termion::event::Key::*;
        match evt {
            Event::Key(ke) => {
                match ke {
                    Char('\n') => self.toggle_collapsed(),
                    Char('\t') => self.create_child(),
                    Delete => self.delete_selected(),
                    Ctrl('e') => self.exec_selected(),
                    Ctrl('l') => self.toggle_show_logs(),
                    Ctrl('f') => self.toggle_hide_stricken(),
                    Ctrl('x') => self.toggle_stricken(),
                    Ctrl('a') => self.draw_arrow(),
                    Ctrl('o') => self.drill_down(),
                    Ctrl('t') => self.pop_focus(),
                    Ctrl('q') => self.auto_arrange(),
                    Alt('\u{1b}') | Ctrl('c') | Ctrl('d') => return false,
                    Ctrl('s') | Ctrl('w') => self.save(),
                    Up => self.select_up(),
                    Down => self.select_down(),
                    Left => self.select_left(),
                    Right => self.select_right(),
                    Backspace => self.backspace(),
                    Char(c) => {
                        if self.last_selected.is_some() {
                            self.append(c);
                        } else {
                            // match c {
                            // 'h' => self.help_screen(),
                            // 'l' => self.log_screen(),
                            // 'm' => self.map_screen(),
                            // 't' => self.task_screen(),
                            // 'g' => self.graph_screen(),
                            // }
                        }
                    }
                    _ => warn!("Weird event {:?}", evt),
                }
            }
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => self.click((x, y)),
                    MouseEvent::Release(x, y) => self.release((x, y)),
                    MouseEvent::Hold(..) => {
                        // this isn't supported in some terminals
                        // (urxvt...) so don't rely on it
                    }
                }
            }
            e => warn!("Weird event {:?}", e),
        }
        true
    }

    fn exec_selected(&mut self) {
        if let Some(selected_id) = self.last_selected {
            let content = self.with_node(selected_id, |n| n.content.clone())
                .unwrap();
            debug!("executing command: {}", content);
            let mut split: Vec<&str> = content.split_whitespace().collect();
            if split.is_empty() {
                debug!("cannot execute empty command");
            }
            let head = split.remove(0);

            if head.starts_with("txt:") {
                self.exec_text_editor(selected_id);
                return;
            }

            let output = if head.starts_with("http") {
                process::Command::new("firefox")
                    .arg(head)
                    .output()
                    .expect(&*format!("command failed to start: {}", content))
            } else {
                process::Command::new(head)
                    .args(&split[..])
                    .output()
                    .expect(&*format!("command failed to start: {}", content))
            };
            log_cmd_output(output);
        }
    }

    fn exec_text_editor(&mut self, node_id: NodeID) {
        let text = self.with_node(node_id, |n| n.free_text.clone())
            .unwrap()
            .unwrap_or("".to_owned());

        // TODO add PID to path
        let path = "/tmp/climate_buffer.tmp";
        debug!("trying to open {} in editor", path);

        // remove old tmp file
        if let Ok(_) = remove_file(&path) {
            // trace!("removed stale tmp file");
        }

        // create new tmp file
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .unwrap();
        f.write_all(text.as_bytes()).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();

        // have raw mode destructor run
        self.cleanup();

        // open text editor
        let ed = env::var("EDITOR").unwrap_or("vim".to_owned());
        process::Command::new(ed)
            .arg(path)
            .spawn()
            .expect(&*format!("failed to open text editor"))
            .wait();

        // read new data
        let mut data = vec![];
        {
            let mut f = File::open(path).unwrap();
            f.read_to_end(&mut data).unwrap();
            // f closed as it slides out of scope
        }
        let new_text = String::from_utf8(data).unwrap();

        remove_file(&path).unwrap();

        // set node's saved text
        self.with_node_mut(node_id, |n| n.free_text = Some(new_text.clone())).unwrap();

        // restore raw mode
        self.start_raw_mode();
    }

    // recursively draw node and children, returning how many have been drawn
    fn draw_node(&mut self,
                 node_id: NodeID,
                 prefix: String,
                 coords: Coords,
                 last: bool,
                 hide_stricken: bool)
                 -> usize {
        // trace!("drawing node {}", node_id);
        let (x, y) = coords;
        let node = self.with_node(node_id, |n| n.clone()).unwrap();
        if node.stricken && hide_stricken {
            return 0;
        }
        let (width, bottom) = terminal_size().unwrap();
        if bottom <= y {
            return 0;
        }
        print!("{}", cursor::Goto(x, y));
        if node.selected {
            print!("{}", style::Invert);
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
        if node.stricken {
            print!("☠");
        } else if node.collapsed {
            print!("⊞");
        } else if node.hide_stricken {
            print!("⚔");
        } else if prefix == "" {
            print!("⚒");
        } else {
            print!(" ");
        }
        if prefix == "" {
            print!(" ");
        }

        print!("{}", node.content);

        println!("{}", style::Reset);
        self.drawn_at.insert(node_id, (x, y));
        for x in (x..(x + 3 + prefix.len() as u16 + node.content.len() as u16)).rev() {
            // trace!("inserting {:?} at {:?}", node_id, (x, y));
            self.lookup.insert((x, y), node_id);
        }
        let mut prefix = prefix;
        if last {
            prefix.push_str("   ");
        } else if prefix == "" {
            prefix.push_str("  ");
        } else {
            prefix.push_str("│  ");
        }
        let prefix = prefix;

        let mut drawn = 1;
        if !node.collapsed {
            let n_children = node.children.len();
            for (n, &child) in node.children
                .iter()
                .enumerate() {
                let last = n + 1 == n_children;
                let child_coords = (x, y + drawn as u16);
                let child_drew = self.draw_node(child,
                                                prefix.clone(),
                                                child_coords,
                                                last,
                                                node.hide_stricken);
                drawn += child_drew;
            }
        }
        drawn
    }

    fn auto_arrange(&mut self) {
        // trace!("auto_arrange");
        let nodes = self.with_node(self.drawing_root, |n| n.children.clone()).unwrap();
        let (width, bottom) = terminal_size().unwrap();
        let between_x = Range::new(1, width);
        let between_y = Range::new(2, bottom - 7);
        let mut rng = rand::thread_rng();
        for node_id in nodes {
            let (mut x, mut y) = (1, 2);
            for _ in 1..20 {
                // try 20 times to place in non-overlapping way
                if self.lookup((x, y)).is_none() {
                    // seems to be empty
                    // TODO test this for children
                    break;
                }
                x = between_x.ind_sample(&mut rng);
                y = between_y.ind_sample(&mut rng);
            }
            self.with_node_mut(node_id, |n| n.rooted_coords = (x, y)).unwrap();
        }
    }

    fn draw_from_root(&mut self) {
        self.lookup.clear();
        self.drawn_at.clear();
        let anchors = self.with_node(self.drawing_root, |n| n.children.clone()).unwrap();
        // trace!("drawing children of root({}): {:?}", self.drawing_root, anchors);
        for child_id in anchors {
            let coords = self.with_node(child_id, |n| n.rooted_coords).unwrap();
            let hide_stricken = self.with_node(self.drawing_root, |n| n.hide_stricken).unwrap();
            self.draw_node(child_id, "".to_owned(), coords, false, hide_stricken);
        }
    }

    fn recursive_child_filter_map<F, B>(&self, node_id: NodeID, mut filter_map: &mut F) -> Vec<B>
        where F: FnMut(&Node) -> Option<B>
    {
        let mut ret = vec![];

        if let Some(node) = self.nodes.get(&node_id) {
            if let Some(b) = filter_map(node) {
                ret.push(b);
            }
            for &child_id in &node.children {
                ret.append(&mut self.recursive_child_filter_map(child_id, filter_map));
            }
        } else {
            debug!("queried for node {} but it is not in self.nodes", node_id);
        }

        ret
    }

    fn print_header(&mut self) {
        let mut header_text = self.with_node(self.drawing_root, |node| node.content.clone())
            .unwrap();

        let now = time::get_time().sec as u64;
        let day_in_sec = 60 * 60 * 24;
        let last_week = now - (day_in_sec * 7);
        let tasks_finished_in_last_week = self.recursive_child_filter_map(self.drawing_root,
                                                                          &mut |n: &Node| {
            let f = n.meta.finish_time;
            if let Some(t) = f {
                if t > last_week {
                    Some(t)
                } else {
                    None
                }
            } else {
                None
            }
        });
        let mut counts = BTreeMap::new();
        for d in 0..7 {
            let t = now - (d * day_in_sec);
            let normalized_t = t / day_in_sec * day_in_sec;
            counts.insert(normalized_t, 0);
        }
        for t in &tasks_finished_in_last_week {
            let normalized_t = t / day_in_sec * day_in_sec;
            let cur = counts.remove(&normalized_t).unwrap_or(0);
            counts.insert(normalized_t, cur + 1);
        }
        let today_normalized = now / day_in_sec * day_in_sec;
        let counts_clone = counts.clone();
        let finished_today = counts_clone.get(&today_normalized).unwrap();
        let week_line = counts.into_iter().map(|(_, v)| v).collect();
        let plot = plot_sparkline(week_line);
        let plot_line = format!("│{}│({} today)", plot, finished_today);
        header_text.push_str(&*plot_line);


        let (width, bottom) = terminal_size().unwrap();
        if width > header_text.len() as u16 && bottom > 1 {
            let mut sep = format!("{}{}{}{}",
                                  cursor::Goto(0, 1),
                                  style::Invert,
                                  header_text,
                                  style::Reset);
            for _ in 0..(width as usize - header_text.len()) {
                sep.push('█');
            }
            println!("{}", sep);
        }


    }

    fn draw(&mut self) {
        print!("{}", clear::All);
        // print header
        self.print_header();

        // print visible nodes
        self.draw_from_root();

        // print logs
        let (width, bottom) = terminal_size().unwrap();
        if self.show_logs && width > 4 && bottom > 7 {
            let mut sep = format!("{}{}logs{}",
                                  cursor::Goto(0, bottom - 6),
                                  style::Invert,
                                  style::Reset);
            for _ in 0..width - 4 {
                sep.push('█');
            }
            println!("{}", sep);
            {
                let logs = logging::read_logs();
                for msg in logs.iter().rev() {
                    let line_width = cmp::min(msg.len(), width as usize);
                    println!("\r{}", msg[..line_width as usize].to_owned());
                }
            }
        }

        // print arrows
        for &(ref from, ref to) in &self.arrows {
            let (path, (d1, d2)) = self.path_between_nodes(*from, *to);
            self.draw_path(path, d1, d2);
        }

        print!("{}", cursor::Hide);
        self.flush();
    }

    fn flush(&mut self) {
        if let Some(mut s) = self.stdout.take() {
            s.flush().unwrap();
            self.stdout = Some(s);
        }
    }

    fn pop_selected(&mut self) -> Option<NodeID> {
        if self.dragging_from.is_none() {
            if let Some(selected_id) = self.last_selected.take() {
                // trace!("popping selected");
                self.with_node_mut(selected_id, |mut node| node.selected = false)
                    .map(|_| selected_id)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn try_select(&mut self, coords: Coords) -> Option<NodeID> {
        // trace!("trying_select({:?}", coords);
        if self.dragging_from.is_none() {
            if let Some(&node_id) = self.lookup(coords) {
                return self.with_node_mut(node_id, |mut node| {
                        // trace!("selected node {} at {:?}", node_id, coords);
                        node.selected = true;
                        node_id
                    })
                    .and_then(|id| {
                        self.last_selected = Some(node_id);
                        self.dragging_from = Some(coords);
                        Some(id)
                    })
                    .or_else(|| {
                        // trace!("found no node at {:?}", coords);
                        None
                    });
            }
        }
        // trace!("selected no node at {:?}", coords);
        // //trace!("lookup is {:?}", self.lookup);
        None
    }

    fn toggle_stricken(&mut self) {
        if let Some(selected_id) = self.last_selected {
            // trace!("toggle stricken");
            self.with_node_mut(selected_id, |node| node.toggle_stricken());
        }
    }

    fn toggle_hide_stricken(&mut self) {
        if let Some(selected_id) = self.last_selected {
            // trace!("toggle hide stricken");
            self.with_node_mut(selected_id, |node| node.toggle_hide_stricken());
        }
    }

    fn delete_recursive(&mut self, node_id: NodeID) {
        if let Some(node) = self.nodes.remove(&node_id) {
            // clean up any arrow state
            self.arrows.retain(|&(ref from, ref to)| from != &node_id && to != &node_id);

            for child_id in &node.children {
                self.delete_recursive(*child_id);
            }
        }
    }

    fn delete_selected(&mut self) {
        if let Some(selected_id) = self.last_selected.take() {
            let coords = self.drawn_at.remove(&selected_id);
            // remove ref from parent
            let parent_id = self.parent(selected_id).unwrap();
            debug!("deleting node {} from parent {}", selected_id, parent_id);
            self.with_node_mut(parent_id, |p| p.children.retain(|c| c != &selected_id)).unwrap();
            // remove children
            self.delete_recursive(selected_id);
            if let Some(c) = coords {
                self.click_select(c);
            }
        }
    }

    pub fn run(&mut self) {
        self.start_raw_mode();
        self.draw();
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();
            let should_break = !self.handle_event(evt);
            self.draw();

            // trace!("nodes:");
            for (id, node) in &self.nodes {
                // trace!("{} -> {:?} -> {}", id, node.children, node.parent_id);
            }

            if should_break {
                self.cleanup();
                self.save();
                break;
            }
        }
    }

    fn toggle_collapsed(&mut self) {
        if let Some(selected_id) = self.last_selected {
            // trace!("collapsed toggle");
            self.with_node_mut(selected_id, |node| node.toggle_collapsed());
        }
    }

    fn toggle_show_logs(&mut self) {
        self.show_logs = !self.show_logs;
    }

    fn create_child(&mut self) {
        if let Some(selected_id) = self.last_selected {
            let node_id = self.new_node();
            self.with_node_mut(node_id, |node| node.parent_id = selected_id);
            let added = self.with_node_mut(selected_id, |selected| {
                selected.children.push(node_id);
            });
            if added.is_some() {
                self.select_node(node_id);
            } else {
                self.delete_recursive(node_id);
            }
        }
    }

    fn create_anchor(&mut self, coords: Coords) {
        let root = self.drawing_root;
        let node_id = self.new_node();
        self.with_node_mut(node_id, |node| {
            node.rooted_coords = coords;
            node.parent_id = root;
        });
        self.with_node_mut(root, |root| root.children.push(node_id));
        // need to draw here to populate drawn_at for select to work below
        self.draw();
        self.try_select(coords);
    }

    fn backspace(&mut self) {
        if let Some(selected_id) = self.last_selected {
            // trace!("backspace");
            self.with_node_mut(selected_id, |node| {
                let newlen = std::cmp::max(node.content.len(), 1) - 1;
                node.content = node.content.clone()[..newlen].to_owned();
            });
        }
    }

    fn append(&mut self, c: char) {
        if let Some(selected_id) = self.last_selected {
            // trace!("append");
            self.with_node_mut(selected_id, |node| {
                node.content.push(c);
            });
        }
    }

    fn drawn_at(&self, node_id: NodeID) -> Option<&Coords> {
        self.drawn_at.get(&node_id)
    }

    fn lookup(&self, coords: Coords) -> Option<&NodeID> {
        self.lookup.get(&coords)
    }

    // returns true if a is a parent of b
    fn is_parent(&self, a: NodeID, b: NodeID) -> bool {
        let mut ptr = b;
        loop {
            // trace!("loop in is_parent");
            if ptr == a {
                return true;
            } else if ptr == 0 {
                // we've reached the top, and 0 is not the parent (a)
                // because we did not return in the last clause
                return false;
            }
            ptr = self.parent(ptr).unwrap();
        }
    }

    fn anchor(&self, node_id: NodeID) -> Result<NodeID, String> {
        if let None = self.drawn_at(node_id) {
            return Err("node not drawn on this screen".to_owned());
        }

        // find the "root" just below self.drawing_root to mod
        // the rooted_coords for.
        let mut ptr = node_id;
        loop {
            let id = self.parent(ptr)?;
            // trace!("anchor loop id: {} ptr: {} selected: {} root: {}", id, ptr, node_id, self.drawing_root);
            if id != self.drawing_root {
                ptr = id;
            } else {
                break;
            }
        }
        Ok(ptr)
    }

    fn parent(&self, node_id: NodeID) -> Result<NodeID, String> {
        self.with_node(node_id, |n| n.parent_id).ok_or("node not found".to_owned())
    }

    fn move_selected(&mut self, from: Coords, to: Coords) {
        let dx = to.0 as i16 - from.0 as i16;
        let dy = to.1 as i16 - from.1 as i16;

        let selected_id = if let Some(selected_id) = self.last_selected {
            if self.is_parent(self.drawing_root, selected_id) {
                selected_id
            } else {
                // selected node is not a child of drawing_root
                debug!("selected node is not child of drawing_root");
                return;
            }
        } else {
            // nothing to drag, no work to do
            return;
        };
        if let Some(&new_parent) = self.lookup(to) {
            if !self.is_parent(selected_id, new_parent) {
                // reparent selected to new_parent
                // 1. remove from old parent's children
                // 2. add to new parent's children
                // 3. set parent_id pointer
                let old_parent = self.parent(selected_id).unwrap();
                self.with_node_mut(old_parent, |op| op.children.retain(|c| c != &selected_id))
                    .unwrap();
                self.with_node_mut(new_parent, |np| np.children.push(selected_id)).unwrap();
                self.with_node_mut(selected_id, |s| s.parent_id = new_parent).unwrap();
            } else {
                let ptr = self.anchor(selected_id).unwrap();
                // trace!("move selected 2");
                self.with_node_mut(ptr, |mut root| {
                        let coords = root.rooted_coords;
                        let nx = cmp::max(coords.0 as i16 + dx, 1) as u16;
                        let ny = cmp::max(coords.1 as i16 + dy, 1) as u16;
                        root.rooted_coords = (nx, ny);
                    })
                    .unwrap();
            }
        } else {
            // destination is not another node, so redraw selected at coords
            // 1. remove from old parent's children
            // 2. add to drawing_root's children
            // 3. update rooted_coords
            let old_parent = self.parent(selected_id).unwrap();
            self.with_node_mut(old_parent, |op| op.children.retain(|c| c != &selected_id)).unwrap();
            let root = self.drawing_root;
            self.with_node_mut(root, |dr| dr.children.push(selected_id)).unwrap();
            self.with_node_mut(selected_id, |s| {
                    let coords = s.rooted_coords;
                    let nx = cmp::max(coords.0 as i16 + dx, 1) as u16;
                    let ny = cmp::max(coords.1 as i16 + dy, 1) as u16;
                    s.rooted_coords = (nx, ny);
                    s.parent_id = root;
                })
                .unwrap();
        }
    }

    fn pop_focus(&mut self) {
        let parent_id = self.parent(self.drawing_root).unwrap();
        self.drawing_root = parent_id;
        self.draw();
    }

    fn drill_down(&mut self) {
        if let Some(selected_id) = self.last_selected {
            self.drawing_root = selected_id;
            self.draw();
        }
    }

    fn click_select(&mut self, coords: Coords) -> Option<NodeID> {
        // trace!("click_select({:?})", coords);
        self.pop_selected();
        let result = self.try_select((coords.0, coords.1));
        self.dragging_from.take();
        result
    }

    fn select_up(&mut self) {
        let node_id = self.find_node(|cur, other| cur.1 > other.1);
        self.select_node(node_id);
    }

    fn select_down(&mut self) {
        let node_id = self.find_node(|cur, other| cur.1 < other.1);
        self.select_node(node_id);
    }

    fn select_left(&mut self) {
        let node_id = self.find_node(|cur, other| cur.0 > other.0);
        self.select_node(node_id);
    }

    fn select_right(&mut self) {
        let node_id = self.find_node(|cur, other| cur.0 < other.0);
        self.select_node(node_id);
    }

    fn find_node<F>(&mut self, sort_fn: F) -> NodeID
        where F: Fn(Coords, Coords) -> bool
    {
        let (width, bottom) = terminal_size().unwrap();
        let selected_id = self.last_selected.unwrap_or(0);
        let default_coords = (width / 2u16, bottom / 2u16);
        let cur = self.drawn_at(selected_id).unwrap_or(&default_coords);
        let (id, _) = self.drawn_at
            .iter()
            .filter_map(|(&n, &nc)| {
                if sort_fn(*cur, nc) {
                    Some((n, cost(*cur, nc)))
                } else {
                    None
                }
            })
            .fold((0, 0), |(acc_id, acc_cost), (id, cost)| {
                if acc_cost == 0 || cost < acc_cost {
                    (id, cost)
                } else {
                    (acc_id, acc_cost)
                }
            });
        id
    }

    fn select_node(&mut self, node_id: NodeID) {
        self.pop_selected();
        // trace!("select_node");
        self.with_node_mut(node_id, |mut node| node.selected = true);
        self.last_selected = Some(node_id);
    }

    fn click(&mut self, coords: Coords) {
        let (x, y) = coords;
        let old = self.pop_selected();
        self.try_select((x, y));
        if old.is_none() && self.dragging_from.is_none() {
            self.create_anchor((x, y));
        }
    }

    fn release(&mut self, coords: Coords) {
        let (x, y) = coords;
        if let Some((from_x, from_y)) = self.dragging_from.take() {
            self.move_selected((from_x, from_y), (x, y));
        }
    }

    fn save(&self) {
        let data = serialization::serialize_screen(self);
        if let Some(ref path) = self.work_path {
            let mut tmp_path = path.clone();
            tmp_path.push_str(".tmp");
            if let Ok(_) = remove_file(&tmp_path) {
                // trace!("removed stale tmp file");
            }
            let mut f = File::create(&tmp_path).unwrap();
            f.write_all(&*data).unwrap();
            f.sync_all().unwrap();
            rename(tmp_path, path).unwrap();
            info!("saved work to {}", path);
        }
    }

    fn cleanup(&mut self) {
        let (_, bottom) = terminal_size().unwrap();
        print!("{}", cursor::Goto(0, bottom));
        println!("{}", cursor::Show);
        self.stdout.take().unwrap().flush().unwrap();
    }

    fn start_raw_mode(&mut self) {
        if self.stdout.is_none() {
            self.stdout = Some(MouseTerminal::from(stdout().into_raw_mode().unwrap()));
        }
    }

    fn occupied(&self, coords: Coords) -> bool {
        self.lookup.contains_key(&coords)
    }

    fn draw_arrow(&mut self) {
        if let Some(from) = self.drawing_arrow.take() {
            if let Some(arrow) = self.last_selected.map(|to| (from, to)) {
                let contains = self.arrows.iter().fold(false, |acc, &(ref nl1, ref nl2)| {
                    if nl1 == &arrow.0 && nl2 == &arrow.1 {
                        true
                    } else {
                        false || acc
                    }
                });
                if contains {
                    self.arrows.retain(|e| e != &arrow);
                } else {
                    self.arrows.push(arrow);
                }
            }
        } else {
            self.drawing_arrow = self.last_selected;
        }
    }

    fn path(&self, start: Coords, dest: Coords) -> Vec<Coords> {
        let (width, bottom) = terminal_size().unwrap();
        if start.0 >= width || dest.0 >= width || start.1 >= bottom || dest.1 >= bottom {
            // trace!("coordinate for arrow is off-screen, returning no path");
            return vec![];
        }
        // trace!("path({:?}, {:?} (screen size: {} x {})", start, dest, width, bottom);
        fn perms(c: Coords) -> Vec<Coords> {
            vec![(c.0 + 1, c.1),
                 (cmp::max(c.0, 1) - 1, c.1),
                 (c.0, c.1 + 1),
                 // we ensure Y is >= 1, since Goto will panic otherwise
                 (c.0, cmp::max(c.1, 2) - 1)]
        }
        // maps from location to previous location
        let mut visited: HashMap<Coords, Coords> = HashMap::new();
        let mut pq = PrioQueue::default();

        // TODO start with dest, go backwards, that way we don't need to reverse
        // draw tree greedily
        let mut cursor = start;
        // trace!("starting draw");
        while cursor != dest {
            for neighbor in perms(cursor) {
                if (!(neighbor.0 >= width) && !(neighbor.1 >= bottom) &&
                    !self.occupied(neighbor) || neighbor == dest) &&
                   !visited.contains_key(&neighbor) {
                    let c = cost(neighbor, dest);
                    pq.insert(c, neighbor);
                    visited.insert(neighbor, cursor);
                }
            }
            if let Some(c) = pq.pop() {
                cursor = c;
            } else {
                // trace!("no path, possible node overlap");
                return vec![];
            }
            // for tracing: show entire search path
            // self.draw_path(visited.clone().keys().map(|k| *k).collect());
        }
        // trace!("done draw, starting backtrack");

        let mut back_cursor = dest;
        let mut path = vec![dest];
        while back_cursor != start {
            let prev = visited.get(&back_cursor).unwrap();
            path.push(*prev);
            back_cursor = *prev;
        }
        path.reverse();
        // trace!("leaving path()");
        path
    }

    fn draw_path(&self, path: Vec<Coords>, start_dir: Dir, dest_dir: Dir) {
        print!("{}", random_color());
        if path.len() == 1 {
            print!("{} ↺", cursor::Goto(path[0].0, path[0].1))
        } else if path.len() > 1 {
            let first = if path[1].1 > path[0].1 {
                match start_dir {
                    Dir::R => '┐',
                    Dir::L => '┌',
                }
            } else if path[1].1 < path[0].1 {
                match start_dir {
                    Dir::R => '┘',
                    Dir::L => '└',
                }
            } else {
                '─'
            };

            print!("{}{}", cursor::Goto(path[0].0, path[0].1), first);
            for items in path.windows(3) {
                let (p, this, n) = (items[0], items[1], items[2]);
                let c = if p.0 == n.0 {
                    '│'
                } else if p.1 == n.1 {
                    '─'
                } else if (this.1 < p.1 && this.0 < n.0) || (this.0 < p.0 && this.1 < n.1) {
                    '┌' // up+right or left+down
                } else if (this.0 > p.0 && this.1 > n.1) || (this.1 > p.1 && this.0 > n.0) {
                    '┘' // right+up or down+left
                } else if (this.0 > p.0 && this.1 < n.1) || (this.1 < p.1 && this.0 > n.0) {
                    '┐' // right+down or up+left
                } else {
                    '└' // down+right or left+up
                };

                print!("{}{}", cursor::Goto(this.0, this.1), c)
            }
            let (end_x, end_y) = (path[path.len() - 1].0, path[path.len() - 1].1);
            let end_char = match dest_dir {
                Dir::L => '>',
                Dir::R => '<',
            };
            print!("{}{}", cursor::Goto(end_x, end_y), end_char);
        }
        print!("{}", color::Fg(color::Reset));
    }

    fn path_between_nodes(&self, start: NodeID, to: NodeID) -> (Vec<Coords>, (Dir, Dir)) {
        // trace!("getting path between {} and {}", start, to);
        let startbounds = self.bounds_for_lookup(start);
        let tobounds = self.bounds_for_lookup(to);
        if startbounds.is_none() || tobounds.is_none() {
            // trace!("path_between_nodes exiting early, point not drawn");
            return (vec![], (Dir::R, Dir::R));
        }
        let (s1, s2) = startbounds.unwrap();
        let (t1, t2) = tobounds.unwrap();

        let init = (self.path(s2, t2), (Dir::R, Dir::R));
        let paths = vec![
            (self.path(s1, t2), (Dir::L, Dir::R)),
            (self.path(s2, t1), (Dir::R, Dir::L)),
            (self.path(s1, t1), (Dir::L, Dir::L)),
        ];
        paths.into_iter()
            .fold(init, |(spath, sdirs), (path, dirs)| {
                if path.len() < spath.len() {
                    (path, dirs)
                } else {
                    (spath, sdirs)
                }
            })
    }

    // correctness depends on invariant of the leftmost element being the
    // value in self.drawn_at
    fn bounds_for_lookup(&self, node_id: NodeID) -> Option<(Coords, Coords)> {
        if let Some(&left) = self.drawn_at(node_id) {
            let mut rx = left.0;
            while let Some(&cursor) = self.lookup((rx + 1, left.1)) {
                if cursor == node_id {
                    rx += 1;
                } else {
                    break;
                }
            }
            // intentionally add 1 to the right side to prevent cluttering
            let right = (rx + 1, left.1);
            Some((left, right))
        } else {
            None
        }
    }
}

fn cost(c1: Coords, c2: Coords) -> u16 {
    let xcost = cmp::max(c1.0, c2.0) - cmp::min(c1.0, c2.0);
    let ycost = cmp::max(c1.1, c2.1) - cmp::min(c1.1, c2.1);
    xcost + ycost
}

fn log_cmd_output(output: process::Output) {
    debug!("status: {}", output.status);
    debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

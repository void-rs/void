use std;
use std::env;
use std::cmp;
use std::fs::{File, rename, remove_file, OpenOptions};
use std::collections::HashMap;
use std::process;
use std::io::{Write, Read, Seek, SeekFrom, Stdout, stdout, stdin};

use termion::{terminal_size, cursor};
use termion::event::{Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

use rand;
use rand::distributions::{IndependentSample, Range};

use mindmap::{cost, NodeID, Coords, Node, serialization, Renderer};

pub struct Screen {
    pub max_id: u64,
    pub drawing_root: NodeID,
    pub nodes: HashMap<NodeID, Node>,
    pub arrows: Vec<(NodeID, NodeID)>,
    pub work_path: Option<String>,
    pub show_logs: bool,
    pub show_meta: bool,
    pub last_selected: Option<NodeID>,
    pub drawing_arrow: Option<NodeID>,
    dragging_from: Option<Coords>,
    pub lookup: HashMap<Coords, NodeID>,
    pub drawn_at: HashMap<NodeID, Coords>,
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

    pub fn with_node<B, F>(&self, k: NodeID, mut f: F) -> Option<B>
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
                    Ctrl('a') => self.add_or_remove_arrow(),
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
                            match c {
                                // 'h' => self.help_screen(),
                                'l' => self.toggle_show_logs(),
                                // 'm' => self.map_screen(),
                                // 't' => self.task_screen(),
                                // 'g' => self.graph_screen(),
                                _ => warn!("Weird event {:?}", evt),
                            }
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
            info!("executing command: {}", content);
            let mut split: Vec<&str> = content.split_whitespace().collect();
            if split.is_empty() {
                error!("cannot execute empty command");
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
            warn!("removed stale tmp file");
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
            .expect("failed to open text editor")
            .wait()
            .unwrap();

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

    fn auto_arrange(&mut self) {
        trace!("auto_arrange");
        let nodes = self.with_node(self.drawing_root, |n| n.children.clone()).unwrap();
        let (width, bottom) = terminal_size().unwrap();
        let between_x = Range::new(1, width);
        let between_y = Range::new(2, bottom - 7);
        let mut rng = rand::thread_rng();
        for node_id in nodes {
            let (mut x, mut y) = (0, 0);
            for _ in 1..20 {
                // try 20 times to place in non-overlapping way
                x = between_x.ind_sample(&mut rng);
                y = between_y.ind_sample(&mut rng);
                if self.lookup((x, y)).is_none() {
                    // seems to be empty
                    // TODO test this for children
                    break;
                }
            }
            self.with_node_mut(node_id, |n| n.rooted_coords = (x, y)).unwrap();
        }
    }

    pub fn recursive_child_filter_map<F, B>(&self,
                                            node_id: NodeID,
                                            mut filter_map: &mut F)
                                            -> Vec<B>
        where F: FnMut(&Node) -> Option<B>
    {
        trace!("recursive_child_filter_map({}, F...)", node_id);
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

    pub fn flush(&mut self) {
        trace!("flush()");
        if let Some(mut s) = self.stdout.take() {
            s.flush().unwrap();
            self.stdout = Some(s);
        }
    }

    fn unselect(&mut self) -> Option<NodeID> {
        trace!("unselect()");
        if self.dragging_from.is_none() {
            if let Some(selected_id) = self.last_selected.take() {
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
        trace!("try_select({:?})", coords);
        if self.dragging_from.is_none() {
            if let Some(&node_id) = self.lookup(coords) {
                return self.with_node_mut(node_id, |mut node| {
                        trace!("selected node {} at {:?}", node_id, coords);
                        node.selected = true;
                        node_id
                    })
                    .and_then(|id| {
                        self.last_selected = Some(node_id);
                        self.dragging_from = Some(coords);
                        Some(id)
                    })
                    .or_else(|| {
                        trace!("found no node at {:?}", coords);
                        None
                    });
            }
        }
        trace!("selected no node at {:?}", coords);
        // //trace!("lookup is {:?}", self.lookup);
        None
    }

    fn toggle_stricken(&mut self) {
        trace!("toggle_stricken()");
        if let Some(selected_id) = self.last_selected {
            self.with_node_mut(selected_id, |node| node.toggle_stricken());
        }
    }

    fn toggle_hide_stricken(&mut self) {
        trace!("toggle_hide_stricken()");
        if let Some(selected_id) = self.last_selected {
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
        trace!("delete_selected()");
        if let Some(selected_id) = self.last_selected.take() {
            let coords = self.drawn_at.remove(&selected_id);
            // remove ref from parent
            let parent_id = self.parent(selected_id).unwrap();
            trace!("deleting node {} from parent {}", selected_id, parent_id);
            self.with_node_mut(parent_id, |p| p.children.retain(|c| c != &selected_id)).unwrap();
            // remove children
            self.delete_recursive(selected_id);
            if let Some(c) = coords {
                self.click_select(c);
            }
        }
    }

    fn draw(&mut self) {
        trace!("draw()");
        let (lookup, drawn_at) = Renderer(self).draw();
        self.flush();
        self.lookup = lookup;
        self.drawn_at = drawn_at;
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
            // for (id, node) in &self.nodes {
            // trace!("{} -> {:?} -> {}", id, node.children, node.parent_id);
            // }

            if should_break {
                self.cleanup();
                self.save();
                break;
            }
        }
        trace!("leaving stdin.events() loop");
    }

    fn toggle_collapsed(&mut self) {
        trace!("toggle_collapsed()");
        if let Some(selected_id) = self.last_selected {
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
        trace!("backspace");
        if let Some(selected_id) = self.last_selected {
            self.with_node_mut(selected_id, |node| {
                let newlen = std::cmp::max(node.content.len(), 1) - 1;
                node.content = node.content.clone()[..newlen].to_owned();
            });
        }
    }

    fn append(&mut self, c: char) {
        trace!("append({})", c);
        if let Some(selected_id) = self.last_selected {
            self.with_node_mut(selected_id, |node| {
                node.content.push(c);
            });
        }
    }

    pub fn drawn_at(&self, node_id: NodeID) -> Option<&Coords> {
        self.drawn_at.get(&node_id)
    }

    pub fn lookup(&self, coords: Coords) -> Option<&NodeID> {
        self.lookup.get(&coords)
    }

    // returns true if a is a parent of b
    fn is_parent(&self, a: NodeID, b: NodeID) -> bool {
        trace!("is_parent({}, {})", a, b);
        let mut ptr = b;
        loop {
            trace!("loop in is_parent");
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
            trace!("anchor loop id: {} ptr: {} selected: {} root: {}",
                   id,
                   ptr,
                   node_id,
                   self.drawing_root);
            if id != self.drawing_root {
                ptr = id;
            } else {
                break;
            }
        }
        Ok(ptr)
    }

    fn parent(&self, node_id: NodeID) -> Result<NodeID, String> {
        trace!("parent({})", node_id);
        self.with_node(node_id, |n| n.parent_id).ok_or("node not found".to_owned())
    }

    fn move_selected(&mut self, from: Coords, to: Coords) {
        trace!("move_selected({:?}, {:?})", from, to);
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
            trace!("leaving move_selected, no work to do");
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
                trace!("move selected 2");
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
        trace!("leaving move_selected");
    }

    fn pop_focus(&mut self) {
        let parent_id = self.parent(self.drawing_root).unwrap();
        self.drawing_root = parent_id;
    }

    fn drill_down(&mut self) {
        trace!("drill_down()");
        if let Some(selected_id) = self.last_selected.take() {
            self.drawing_root = selected_id;
        }
    }

    fn click_select(&mut self, coords: Coords) -> Option<NodeID> {
        trace!("click_select({:?})", coords);
        self.unselect();
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
        trace!("select_node({})", node_id);
        self.unselect();
        self.with_node_mut(node_id, |mut node| node.selected = true);
        self.last_selected = Some(node_id);
    }

    fn click(&mut self, coords: Coords) {
        let (x, y) = coords;
        let old = self.unselect();
        let new = self.try_select((x, y));
        if old.is_none() && self.dragging_from.is_none() {
            self.create_anchor((x, y));
        }
        if old.is_some() && old == new {
            self.drill_down();
        }
    }

    fn release(&mut self, coords: Coords) {
        trace!("release({:?})", coords);
        let (x, y) = coords;
        if let Some((from_x, from_y)) = self.dragging_from.take() {
            self.move_selected((from_x, from_y), (x, y));
        }
        trace!("leaving release");
    }

    fn save(&self) {
        trace!("save()");
        let data = serialization::serialize_screen(self);
        if let Some(ref path) = self.work_path {
            let mut tmp_path = path.clone();
            tmp_path.push_str(".tmp");
            if let Ok(_) = remove_file(&tmp_path) {
                warn!("removed stale tmp file");
            }
            let mut f = File::create(&tmp_path).unwrap();
            f.write_all(&*data).unwrap();
            f.sync_all().unwrap();
            rename(tmp_path, path).unwrap();
            info!("saved work to {}", path);
        }
    }

    fn cleanup(&mut self) {
        trace!("cleanup()");
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

    pub fn occupied(&self, coords: Coords) -> bool {
        self.lookup.contains_key(&coords)
    }

    pub fn add_or_remove_arrow(&mut self) {
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
}

fn log_cmd_output(output: process::Output) {
    debug!("status: {}", output.status);
    debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

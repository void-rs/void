use std;
use std::env;
use std::cmp;
use std::fs::{File, rename, remove_file, OpenOptions};
use std::collections::{BTreeMap, HashMap, BinaryHeap, HashSet};
use std::process;
use std::io::{self, Write, Read, Seek, SeekFrom, Stdout, stdout, stdin, Error, ErrorKind};
use std::fmt::Write as FmtWrite;

use termion::{terminal_size, color, cursor, style, clear};
use termion::event::{Event, Key};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

use libc::getpid;
use time;
use regex::Regex;
use rand::{self, Rng};

use {Config, Action, cost, NodeID, Coords, Node, random_fg_color, serialization, Dir, Pack};
use plot::plot_sparkline;
use logging;

pub struct Screen {
    pub max_id: u64,
    pub nodes: HashMap<NodeID, Node>,
    pub arrows: Vec<(NodeID, NodeID)>,
    pub work_path: Option<String>,
    pub config: Config,

    // screen dimensions as detected during the current draw() cycle
    pub dims: Coords,
    pub is_test: bool,

    // non-pub members are ephemeral
    drawing_root: NodeID,
    show_logs: bool,
    selected: Option<NodeID>,
    drawing_arrow: Option<NodeID>,
    lookup: HashMap<Coords, NodeID>,
    drawn_at: HashMap<NodeID, Coords>,
    dragging_from: Option<Coords>,
    dragging_to: Option<Coords>,
    stdout: Option<MouseTerminal<RawTerminal<Stdout>>>,
    lowest_drawn: u16,
    // where we start drawing from
    view_y: u16,
    // when we drill down then pop up, we should go to last focus, stored here
    focus_stack: Vec<(NodeID, NodeID, u16)>,
}

impl Default for Screen {
    fn default() -> Screen {
        let mut root = Node::default();
        root.content = "home".to_owned();
        let mut screen = Screen {
            config: Config::default(),
            arrows: vec![],
            selected: None,
            drawing_arrow: None,
            nodes: HashMap::new(),
            lookup: HashMap::new(),
            drawn_at: HashMap::new(),
            show_logs: false,
            drawing_root: 0,
            stdout: None,
            dragging_from: None,
            dragging_to: None,
            work_path: None,
            max_id: 0,
            dims: terminal_size().unwrap(),
            lowest_drawn: 0,
            view_y: 0,
            focus_stack: vec![],
            is_test: false,
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
    pub fn handle_event(&mut self, evt: Event) -> bool {
        match self.config.map(evt) {
            Some(e) => {
                match e {
                    Action::LeftClick(x, y) => {
                        let internal_coords = self.screen_to_internal_xy((x, y));
                        self.click_screen(internal_coords)
                    }
                    Action::Release(x, y) => {
                        let internal_coords = self.screen_to_internal_xy((x, y));
                        self.release(internal_coords)
                    }
                    Action::Char(c) => {
                        if self.selected.is_some() {
                            self.append(c);
                        } else {
                            self.prefix_jump_to(c.to_string());
                        }
                    }
                    Action::UnselectRet => return self.unselect().is_some(),
                    Action::ScrollUp => self.scroll_up(),
                    Action::ScrollDown => self.scroll_down(),
                    Action::DeleteSelected => self.delete_selected(true),
                    Action::SelectUp => self.select_up(),
                    Action::SelectDown => self.select_down(),
                    Action::SelectLeft => self.select_left(),
                    Action::SelectRight => self.select_right(),
                    Action::EraseChar => self.backspace(),
                    Action::CreateSibling => self.create_sibling(),
                    Action::CreateChild => self.create_child(),
                    Action::CreateFreeNode => self.create_free_node(),
                    Action::ExecSelected => self.exec_selected(),
                    Action::DrillDown => self.drill_down(),
                    Action::PopUp => self.pop_focus(),
                    Action::PrefixJump => self.prefix_jump_prompt(),
                    Action::ToggleCompleted => self.toggle_stricken(),
                    Action::ToggleHideCompleted => self.toggle_hide_stricken(),
                    Action::Arrow => self.add_or_remove_arrow(),
                    Action::Arrange => self.arrange(),
                    Action::AutoArrange => self.toggle_auto_arrange(),
                    Action::ToggleCollapsed => self.toggle_collapsed(),
                    Action::Quit => return false,
                    Action::Save => self.save(),
                    Action::ToggleShowLogs => self.toggle_show_logs(),
                    Action::EnterCmd => self.enter_cmd(),
                    Action::FindTask => self.auto_task(),
                    Action::YankPasteNode => self.cut_paste(),
                }
            }
            None => warn!("received unknown input"),
        }
        true
    }

    fn cut_paste(&mut self) {
        if let Some(dragging_from) = self.dragging_from.take() {
            if let Some(dragging_to) = self.dragging_to.take() {
                self.move_selected(dragging_from, dragging_to);
            }
        } else {
            if let Some(selected_id) = self.selected {
                if let Some(&coords) = self.drawn_at(selected_id) {
                    self.dragging_from = Some(coords);
                }
            }
        }
    }

    fn auto_task(&mut self) {
        // find all leaf children of incomplete tasks
        // if a parent is complete, the children are complete
        // if all children are complete, but the parent isn't,
        // we need to finish the parent
        let mut task_roots = vec![];
        let mut to_explore = vec![self.drawing_root];
        while let Some(node_id) = to_explore.pop() {
            let mut node = self.with_node(node_id, |n| n.clone()).unwrap();
            if node.stricken {
                // pass
            } else if node.content.contains("#task") {
                task_roots.push(node.id);
            } else {
                to_explore.append(&mut node.children);
            }
        }

        let mut leaves = vec![];
        while let Some(root_id) = task_roots.pop() {
            let node = self.with_node(root_id, |n| n.clone()).unwrap();
            let mut incomplete_children: Vec<_> = node.children
                .iter()
                .cloned()
                .filter(|&c| self.with_node(c, |c| !c.stricken).unwrap())
                .collect();
            if incomplete_children.is_empty() {
                leaves.push(root_id);
            } else {
                task_roots.append(&mut incomplete_children);
            }
        }

        if leaves.is_empty() {
            info!("no tasks to jump to! create some first");
            return;
        }

        // weight based on priority of most important ancestor

        let mut prio_pairs = vec![];
        let mut total_prio = 0;
        for &leaf in &leaves {
            let prio = self.lineage(leaf)
                .iter()
                .filter_map(|&p| self.node_priority(p))
                .max()
                .unwrap_or(1);
            total_prio += prio;
            prio_pairs.push((prio, leaf));
        }

        let mut idx: usize = rand::thread_rng().gen_range(0, total_prio);

        let mut choice = None;
        for &(prio, leaf) in &prio_pairs {
            if prio > idx {
                choice = Some(leaf);
                break;
            }
            idx -= prio;
        }
        let choice = choice.unwrap();

        // jump to highest view where node is visible
        let mut cursor = choice;
        loop {
            trace!("in auto_task loop");
            let parent = self.parent(cursor).unwrap();
            let collapsed = self.with_node(parent, |p| p.collapsed).unwrap();
            cursor = parent;
            if parent == 0 || collapsed {
                break;
            }
        }

        // save old location and jump
        let old_select = self.unselect().unwrap_or(0);
        let breadcrumb = (self.drawing_root, old_select, self.view_y);
        self.focus_stack.push(breadcrumb);
        self.drawing_root = cursor;
        self.select_node(choice);
    }

    fn node_priority(&self, node_id: NodeID) -> Option<usize> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#prio=(\d+)").unwrap();
        }
        self.with_node(node_id, |n| n.content.clone())
            .and_then(|c| {
                if RE.is_match(&*c) {
                    RE.captures_iter(&*c)
                        .nth(0)
                        .and_then(|n| n.at(1).unwrap().parse::<usize>().ok())
                } else {
                    None
                }
            })
    }

    fn single_key_prompt(&mut self, prompt: &str) -> io::Result<Key> {
        trace!("prompt({})", prompt);
        if self.is_test {
            return Err(Error::new(ErrorKind::Other, "can't prompt in test"));
        }

        let stdin: Box<Read> = Box::new(stdin());
        print!("{}{}{}{}",
               style::Invert,
               cursor::Goto(0, self.dims.1),
               clear::AfterCursor,
               prompt);
        self.flush();
        let res = stdin.keys().nth(0).unwrap();
        debug!("read prompt: {:?}", res);
        res
    }

    fn prompt(&mut self, prompt: &str) -> io::Result<Option<String>> {
        trace!("prompt({})", prompt);
        if self.is_test {
            return Err(Error::new(ErrorKind::Other, "can't prompt in test"));
        }
        let mut stdin: Box<Read> = Box::new(stdin());
        print!("{}{}{}{}{}",
               style::Invert,
               cursor::Goto(0, self.dims.1),
               clear::AfterCursor,
               prompt,
               cursor::Show);
        self.cleanup();
        let res = stdin.read_line();
        self.start_raw_mode();
        debug!("read prompt: {:?}", res);
        res
    }

    fn enter_cmd(&mut self) {
        trace!("enter_cmd()");
        if let Ok(Some(cmd)) = self.prompt("cmd: ") {
            debug!("received command {:?}", cmd);
        }
    }

    fn prefix_jump_prompt(&mut self) {
        trace!("prefix_jump_prompt()");

        let prefix = match self.single_key_prompt("prefix: ") {
            Ok(Key::Char(c)) => c.to_string(),
            _ => return,
        };
        self.prefix_jump_to(prefix)
    }

    fn prefix_jump_to(&mut self, prefix: String) {
        let chars = "arstqwfpgdbvcxzoienyuljhkm1234567890ARSTQWFPGDVCXZOIENYULJHBKM";
        // get visible nodes that contain prefix
        let nodes = self.find_visible_nodes(|node_id| {
            self.with_node(node_id, |n| n.content.starts_with(&*prefix)).unwrap()
        });

        if nodes.is_empty() {
            return;
        } else if nodes.len() == 1 {
            let node_id = nodes[0];
            self.select_node(node_id);
            return;
        }

        // map an alphanumeric char to each candidate NodeID
        let mapping: HashMap<&str, NodeID> =
            chars.split("").skip(1).zip(nodes.into_iter()).collect();
        // print the hilighted char at each choice
        self.draw();
        for (&c, &node_id) in &mapping {
            let &coords = self.drawn_at(node_id).unwrap();
            let (x, y) = self.internal_to_screen_xy(coords).unwrap();
            print!("{}{}{}{}",
                   cursor::Goto(x, y),
                   style::Invert,
                   c,
                   style::Reset);
        }

        // read the choice
        let choice = match self.single_key_prompt("choice: ") {
            Ok(Key::Char(c)) => c.to_string(),
            _ => return,
        };

        // jump or exit
        if let Some(&node_id) = mapping.get(&*choice) {
            debug!("jumping to node {}", node_id);
            self.select_node(node_id);
        }
    }

    fn find_visible_nodes<F>(&self, mut filter: F) -> Vec<NodeID>
        where F: FnMut(NodeID) -> bool
    {
        self.drawn_at
            .keys()
            .filter(|&node_id| self.node_is_visible(*node_id).unwrap())
            .filter(|&node_id| filter(*node_id))
            .cloned()
            .collect()
    }

    fn exec_selected(&mut self) {
        if let Some(selected_id) = self.selected {
            let content_opt = self.with_node(selected_id, |n| n.content.clone());
            if content_opt.is_none() {
                error!("tried to exec deleted node");
                return;
            }
            let content = content_opt.unwrap();
            info!("executing command: {}", content);
            let mut split: Vec<&str> = content.split_whitespace().collect();
            if split.is_empty() {
                error!("cannot execute empty command");
                return;
            }
            let head = split.remove(0);

            if head.starts_with("txt:") {
                self.exec_text_editor(selected_id);
                return;
            }

            if head.starts_with("http") {
                let cmd = process::Command::new("firefox")
                    .arg(head)
                    .spawn();
                if cmd.is_err() {
                    error!("command failed to start: {}", content);
                }
            } else {
                let cmd = process::Command::new(head)
                    .args(&split[..])
                    .spawn();
                if cmd.is_err() {
                    error!("command failed to start: {}", content);
                }
            }
        }
    }

    fn exec_text_editor(&mut self, node_id: NodeID) {
        let text = self.with_node(node_id, |n| n.free_text.clone())
            .unwrap()
            .unwrap_or("".to_owned());

        let pid = unsafe { getpid() };
        let path = format!("/tmp/void_buffer.tmp.{}", pid);
        debug!("trying to open {} in editor", path);

        // remove old tmp file
        if let Ok(_) = remove_file(&path) {
            warn!("removed stale tmp file");
        }

        // create new tmp file
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        f.write_all(text.as_bytes()).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();

        // have raw mode destructor run
        self.cleanup();

        // open text editor
        let ed = env::var("EDITOR").unwrap_or("vim".to_owned());
        process::Command::new(ed)
            .arg(&path)
            .spawn()
            .expect("failed to open text editor")
            .wait()
            .unwrap();

        // read new data
        let mut data = vec![];
        {
            let mut f = File::open(&path).unwrap();
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

    pub fn arrange(&mut self) {
        trace!("arrange");
        let mut real_estate = Pack {
            children: None,
            top: 2, // leave room for header
            left: 1, // 1-indexed screen
            bottom: std::u16::MAX, // make this "bottomless" since we can paginate
            right: self.dims.0 - 1,
            elem: None,
        };

        let nodes = self.with_node(self.drawing_root, |n| n.children.clone()).unwrap();
        let mut node_dims: Vec<(NodeID, Coords)> = nodes.into_iter()
            .map(|n| (n, self.drawable_subtree_dims(n).unwrap()))
            .collect();
        node_dims.sort_by_key(|&(_, (_, y))| y);
        node_dims.reverse();

        for (node_id, dims) in node_dims {
            // add some spacing around this tree to space out
            // placement a little bit
            let padded_dims = (dims.0 + 2, dims.1 + 2);
            let (x, y) = real_estate.insert(padded_dims).unwrap();
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

    fn drawable_subtree_dims(&self, node_id: NodeID) -> Option<(u16, u16)> {
        if let Some(widths) = self.drawable_subtree_widths(node_id, 0) {
            let height = widths.len() as u16;
            let max_width = widths.into_iter().max().unwrap();
            Some((max_width, height))
        } else {
            None
        }
    }

    fn drawable_subtree_widths(&self, node_id: NodeID, depth: usize) -> Option<Vec<u16>> {
        if let Some(node) = self.nodes.get(&node_id) {
            let width = 1 + (3 * depth as u16) + node.content.len() as u16;
            let mut ret = vec![width];
            if !node.collapsed {
                for &child in &node.children {
                    // ASSUMES node.children are all valid
                    let mut child_widths = self.drawable_subtree_widths(child, depth + 1)
                        .unwrap();
                    ret.append(&mut child_widths);
                }
            }
            Some(ret)
        } else {
            None
        }
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
            if let Some(selected_id) = self.selected {
                if let Some(is_empty) = self.with_node(selected_id, |n| n.content.is_empty()) {
                    if is_empty {
                        self.delete_selected(false);
                        return None;
                    } else {
                        self.selected.take();
                        self.with_node_mut(selected_id, |mut node| node.selected = false)
                            .unwrap();
                        return Some(selected_id);
                    }
                }
            }
        }
        None
    }

    fn internal_to_screen_xy(&self, coords: Coords) -> Option<Coords> {
        // + 2 compensates for header
        if coords.1 < self.view_y + 2 || coords.1 > self.view_y + self.dims.1 {
            // coords are above or below screen
            None
        } else {
            Some((coords.0, coords.1 - self.view_y))
        }
    }

    fn screen_to_internal_xy(&self, coords: Coords) -> Coords {
        (coords.0, coords.1 + self.view_y)
    }

    fn coords_are_visible(&self, (_, y): Coords) -> bool {
        cmp::max(y, 1) - 1 > self.view_y && y < self.view_y + self.dims.1 + 1
    }

    fn node_is_visible(&self, node: NodeID) -> Option<bool> {
        if let Some(&coords) = self.drawn_at(node) {
            Some(self.coords_are_visible(coords))
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
                        self.selected = Some(node_id);
                        self.dragging_from = Some(coords);
                        self.dragging_to = Some(coords);
                        Some(id)
                    })
                    .or_else(|| {
                        trace!("found no node at {:?}", coords);
                        None
                    });
            }
        } else {
            self.dragging_to = Some(coords);
        }
        trace!("selected no node at {:?}", coords);
        // //trace!("lookup is {:?}", self.lookup);
        None
    }

    fn toggle_stricken(&mut self) {
        trace!("toggle_stricken()");
        if let Some(selected_id) = self.selected {
            self.with_node_mut(selected_id, |node| node.toggle_stricken());
        }
    }

    fn toggle_hide_stricken(&mut self) {
        trace!("toggle_hide_stricken()");
        if let Some(selected_id) = self.selected {
            self.with_node_mut(selected_id, |node| node.toggle_hide_stricken());
        }
    }

    fn delete_recursive(&mut self, node_id: NodeID) {
        debug!("deleting node {}", node_id);
        if let Some(node) = self.nodes.remove(&node_id) {
            // clean up any arrow state
            self.arrows.retain(|&(ref from, ref to)| from != &node_id && to != &node_id);

            for child_id in &node.children {
                self.delete_recursive(*child_id);
            }
        }
    }

    fn delete_selected(&mut self, reselect: bool) {
        trace!("delete_selected()");
        if let Some(selected_id) = self.selected.take() {
            let coords = self.drawn_at.remove(&selected_id);
            debug!("coords: {:?}", coords);
            // remove ref from parent
            if let Some(parent_id) = self.parent(selected_id) {
                trace!("deleting node {} from parent {}", selected_id, parent_id);
                self.with_node_mut(parent_id, |p| p.children.retain(|c| c != &selected_id))
                    .unwrap();
            }
            // remove children
            self.delete_recursive(selected_id);
            if let Some(c) = coords {
                if reselect {
                    // need to draw here or there will be nothing to click_select below
                    self.draw();
                    self.click_select(c);
                }
            }
        }
    }

    pub fn should_auto_arrange(&self) -> bool {
        self.with_node(self.drawing_root, |n| n.auto_arrange).unwrap()
    }

    fn toggle_auto_arrange(&mut self) {
        let root = self.drawing_root;
        self.with_node_mut(root, |mut n| n.auto_arrange = !n.auto_arrange)
            .unwrap()
    }

    pub fn run(&mut self) {
        self.start_raw_mode();
        self.draw();
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();

            self.dims = terminal_size().unwrap();

            let should_break = !self.handle_event(evt);

            if self.should_auto_arrange() {
                self.arrange();
            }

            self.draw();

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
        if let Some(selected_id) = self.selected {
            self.with_node_mut(selected_id, |node| node.toggle_collapsed());
        }
    }

    fn toggle_show_logs(&mut self) {
        self.show_logs = !self.show_logs;
    }

    fn create_child(&mut self) {
        if let Some(selected_id) = self.selected {
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

    fn create_sibling(&mut self) {
        if let Some(selected_id) = self.selected {
            if let Some(parent_id) = self.parent(selected_id) {
                if parent_id == self.drawing_root {
                    // don't want to deal with this case right now
                    return;
                }
                let node_id = self.new_node();

                self.with_node_mut(node_id, |node| node.parent_id = parent_id);
                let added = self.with_node_mut(parent_id, |parent| {
                    parent.children.push(node_id);
                });
                if added.is_some() {
                    self.select_node(node_id);
                } else {
                    self.delete_recursive(node_id);
                }
            }
        }
    }

    fn create_free_node(&mut self) {
        let min_width = self.dims.0 / 3;
        let mut y_cursor = self.view_y + 2;
        let mut from_x = None;
        'outer: loop {
            trace!("in create_free_node loop");
            // go down until we find a spot
            // that's wide enough, then create
            // an anchor there.
            let mut width = 0;
            for x in 1..self.dims.0 {
                if self.lookup((x, y_cursor)).is_none() {
                    width += 1;
                    if from_x.is_none() {
                        from_x = Some(x)
                    }
                    if width >= min_width {
                        break 'outer;
                    }
                } else {
                    from_x = None;
                    width = 0;
                }
            }
            y_cursor += 1;
        }
        self.create_anchor((from_x.unwrap(), y_cursor));
    }

    fn create_anchor(&mut self, coords: Coords) {
        let root = self.drawing_root;
        let node_id = self.new_node();
        self.with_node_mut(node_id, |node| {
            node.rooted_coords = coords;
            node.parent_id = root;
        });
        self.with_node_mut(root, |root| root.children.push(node_id));
        self.select_node(node_id);
    }

    fn backspace(&mut self) {
        trace!("backspace");
        if let Some(selected_id) = self.selected {
            self.with_node_mut(selected_id, |node| {
                let content = node.content.clone();
                let chars = content.chars();
                let oldlen = chars.clone().count();
                let truncated: String = chars.take(cmp::max(oldlen, 1) - 1).collect();
                node.content = truncated;
            });
        }
    }

    fn append(&mut self, c: char) {
        trace!("append({})", c);
        if let Some(selected_id) = self.selected {
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

    fn lineage(&self, node_id: NodeID) -> Vec<NodeID> {
        let mut lineage = vec![node_id];
        let mut cursor = node_id;
        while let Some(parent) = self.parent(cursor) {
            lineage.push(parent);
            if parent == 0 {
                break;
            }
            cursor = parent;
        }
        lineage.reverse();
        lineage
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
            let id = self.parent(ptr).ok_or("node has no parent")?;
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

    fn parent(&self, node_id: NodeID) -> Option<NodeID> {
        trace!("parent({})", node_id);
        self.with_node(node_id, |n| n.parent_id)
    }

    fn move_selected(&mut self, from: Coords, to: Coords) {
        trace!("move_selected({:?}, {:?})", from, to);
        let dx = to.0 as i16 - from.0 as i16;
        let dy = to.1 as i16 - from.1 as i16;

        let selected_id = if let Some(selected_id) = self.selected {
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
                if self.with_node(new_parent, |np| np.collapsed).unwrap() {
                    // if the destination is collapsed, deselect this node
                    self.unselect();
                }
            } else {
                // we're here because we released the drag
                // with the cursor over a child, so rather
                // than create a cycle, we move the subtree.
                let ptr = self.anchor(selected_id).unwrap();
                trace!("move selected 2");
                self.with_node_mut(ptr, |mut root| {
                        let (ox, oy) = root.rooted_coords;
                        let nx = cmp::max(ox as i16 + dx, 1) as u16;
                        let ny = cmp::max(oy as i16 + dy, 1) as u16;
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
            self.with_node_mut(old_parent, |op| op.children.retain(|c| c != &selected_id))
                .unwrap();
            let root = self.drawing_root;
            self.with_node_mut(root, |dr| dr.children.push(selected_id)).unwrap();
            self.with_node_mut(selected_id, |s| {
                    s.rooted_coords = to;
                    s.parent_id = root;
                })
                .unwrap();
        }
        trace!("leaving move_selected");
    }

    fn pop_focus(&mut self) {
        self.unselect();
        let (root, selected, view_y) = self.focus_stack.pop().unwrap_or((0, 0, 0));
        self.drawing_root = root;
        self.view_y = view_y;
        self.select_node(selected);
    }

    fn drill_down(&mut self) {
        trace!("drill_down()");
        if let Some(selected_id) = self.unselect() {
            if selected_id != self.drawing_root {
                let breadcrumb = (self.drawing_root, selected_id, self.view_y);
                self.focus_stack.push(breadcrumb);
                self.drawing_root = selected_id;
                self.view_y = 0;
            }
        }
    }

    fn click_select(&mut self, coords: Coords) -> Option<NodeID> {
        trace!("click_select({:?})", coords);
        self.unselect();
        let result = self.try_select(coords);
        self.dragging_from.take();
        self.dragging_to.take();
        result
    }

    fn scroll_up(&mut self) {
        self.view_y = cmp::max(self.view_y, self.dims.1 / 2) - self.dims.1 / 2;
        self.unselect();
    }

    fn scroll_down(&mut self) {
        if self.lowest_drawn > self.view_y + self.dims.1 {
            self.view_y = cmp::min(self.view_y + self.dims.1 / 2, self.lowest_drawn);
            self.unselect();
        }
    }

    fn scroll_to_selected(&mut self) -> bool {
        if let Some(selected_id) = self.selected {
            self.scroll_to_node(selected_id)
        } else {
            false
        }
    }

    fn scroll_to_node(&mut self, node_id: NodeID) -> bool {
        if let Some(&(_, y)) = self.drawn_at(node_id) {
            if y <= self.view_y || y > (self.view_y + self.dims.1) {
                // move only if necessary
                self.view_y = cmp::max(y - 1, self.dims.1 / 2) - self.dims.1 / 2;
            }
            true
        } else {
            false
        }
    }

    fn select_up(&mut self) {
        self.select_relative(|cur, other| cur.1 > other.1);
    }

    fn select_down(&mut self) {
        self.select_relative(|cur, other| cur.1 < other.1);
    }

    fn select_left(&mut self) {
        self.select_relative(|cur, other| cur.0 > other.0);
    }

    fn select_right(&mut self) {
        self.select_relative(|cur, other| cur.0 < other.0);
    }

    fn select_relative<F>(&mut self, filter_fn: F)
        where F: Fn(Coords, Coords) -> bool
    {
        if let Some(node_id) = self.find_relative_node(filter_fn) {
            self.select_node(node_id);
        }
    }

    fn find_relative_node<F>(&mut self, filter_fn: F) -> Option<NodeID>
        where F: Fn(Coords, Coords) -> bool
    {
        let selected_id = self.selected.unwrap_or(0);
        let default_coords = (self.dims.0 / 2, self.dims.1 / 2);
        let rel_def_coords = self.screen_to_internal_xy(default_coords);
        let cur = self.drawn_at(selected_id).unwrap_or(&rel_def_coords);
        let (id, _) = self.drawn_at
            .iter()
            .fold((None, std::u16::MAX),
                  |(acc_id, acc_cost), (&node_id, &(x, y))| {
                if filter_fn(*cur, (x, y)) {
                    let cost = cost(*cur, (x, y));
                    if cost < acc_cost {
                        (Some(node_id), cost)
                    } else {
                        (acc_id, acc_cost)
                    }
                } else {
                    (acc_id, acc_cost)
                }
            });
        id
    }

    fn node_cost(&self, node1: NodeID, node2: NodeID) -> Option<u16> {
        if let Some((l1, r1)) = self.bounds_for_lookup(node1) {
            if let Some((l2, r2)) = self.bounds_for_lookup(node2) {
                let possibilities = vec![(l1, l2), (l1, r2), (r1, l2), (r1, r2)];
                return possibilities.into_iter().map(|(one, two)| cost(one, two)).min();
            }
        }
        None
    }

    fn select_node(&mut self, node_id: NodeID) {
        trace!("select_node({})", node_id);
        self.unselect();
        if node_id != 0 {
            self.with_node_mut(node_id, |mut node| node.selected = true);
            self.selected = Some(node_id);

            // draw() needed to make visible / scroll accurate
            self.draw();

            if let Some(visible) = self.node_is_visible(node_id) {
                if !visible {
                    self.scroll_to_selected();
                }
            }
        }
    }

    fn click_screen(&mut self, coords: Coords) {
        if coords.0 > self.dims.0 || coords.1 > self.lowest_drawn {
            warn!("click way off-screen");
            return;
        }
        let old = self.unselect();
        let new = self.try_select(coords);
        if old.is_none() && self.dragging_from.is_none() {
            self.create_anchor(coords);
        }
        if old.is_some() && old == new {
            self.drill_down();
        }
    }

    fn release(&mut self, to: Coords) {
        trace!("release({:?})", to);
        if to.0 > self.dims.0 || to.1 > self.lowest_drawn {
            warn!("release way off-screen");
            return;
        }
        if let Some(from) = self.dragging_from.take() {
            self.dragging_to.take();
            self.move_selected(from, to);
        }
        trace!("leaving release");
    }

    pub fn assert_node_consistency(&self) {
        // a child should be a child of at most one node
        debug!("testing that no nodes have multiple parents");
        let mut seen = HashSet::new();
        let mut to_view = vec![0];
        let mut leaf_children = vec![];
        while let Some(node_id) = to_view.pop() {
            self.with_node(node_id, |n| {
                for &c in &n.children {
                    assert!(!seen.contains(&c));
                    seen.insert(c);
                }
                if n.children.is_empty() {
                    leaf_children.push(node_id);
                }
            });
        }

        // no parent loops
        debug!("testing that 0 is the ancestor of all nodes");
        for &node_id in self.nodes.keys() {
            assert!(self.is_parent(0, node_id));
        }

    }

    pub fn save(&self) {
        trace!("save()");
        self.assert_node_consistency();
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

    pub fn cleanup(&mut self) {
        trace!("cleanup()");
        print!("{}", cursor::Show);
        self.stdout.take().unwrap().flush().unwrap();
    }

    pub fn start_raw_mode(&mut self) {
        if self.stdout.is_none() {
            self.stdout = Some(MouseTerminal::from(stdout().into_raw_mode().unwrap()));
        }
    }

    pub fn occupied(&self, coords: Coords) -> bool {
        self.lookup.contains_key(&coords)
    }

    pub fn add_or_remove_arrow(&mut self) {
        if let Some(from) = self.drawing_arrow.take() {
            if let Some(arrow) = self.selected.map(|to| (from, to)) {
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
            self.drawing_arrow = self.selected;
        }
    }

    // *
    // *
    // *┌──┐
    // *│  Drawing Functionality
    // *└──────────────>
    // *
    // *

    pub fn draw(&mut self) {
        trace!("draw()");
        self.lookup.clear();
        self.drawn_at.clear();
        self.lowest_drawn = 0;

        trace!("draw()");
        print!("{}", clear::All);

        self.draw_header();

        // print visible nodes
        self.draw_children_of_root();

        // print logs
        if self.show_logs && self.dims.0 > 4 && self.dims.1 > 7 {
            let mut sep = format!("{}{}logs{}",
                                  cursor::Goto(0, self.dims.1 - 6),
                                  style::Invert,
                                  style::Reset);
            for _ in 0..self.dims.0 - 4 {
                sep.push('█');
            }
            println!("{}", sep);
            {
                let logs = logging::read_logs();
                for msg in logs.iter().rev() {
                    let line_width = cmp::min(msg.len(), self.dims.0 as usize);
                    println!("\r{}", msg[..line_width as usize].to_owned());
                }
            }
        }

        // print arrows
        for &(ref from, ref to) in &self.arrows {
            let (path, (direction1, direction2)) = self.path_between_nodes(*from, *to);
            self.draw_path(path, direction1, direction2);
        }

        // conditionally print drag dest arrow
        if let Some(from) = self.dragging_from {
            // we only care if we're dragging a node
            if let Some(from_node) = self.lookup(from) {
                // we're either dragging to a new parent node, or to a new space
                if let Some(to) = self.dragging_to {
                    if let Some(to_node) = self.lookup(to) {
                        let (path, (direction1, direction2)) =
                            self.path_between_nodes(*from_node, *to_node);
                        self.draw_path(path, direction1, direction2);
                    } else {
                        let (path, (direction1, direction2)) =
                            self.path_from_node_to_point(*from_node, to);
                        self.draw_path(path, direction1, direction2);
                    }
                } else {
                    warn!("dragging_from set, but NOT dragging_to");
                }
            }
        }

        // show scrollbar if we've drawn anything below the bottom of the screen
        if self.lowest_drawn > self.dims.1 {
            self.draw_scrollbar();
        }

        print!("{}", cursor::Hide);
        self.flush();
    }

    fn draw_scrollbar(&self) {
        let bar_height = self.dims.1 - 1;
        let normalized_lowest = cmp::max(self.lowest_drawn, 1) as f64;
        let fraction_viewable = self.dims.1 as f64 / normalized_lowest;
        let shade_start_fraction = self.view_y as f64 / normalized_lowest;

        let shade_amount = (bar_height as f64 * fraction_viewable) as usize;
        let shade_start = (bar_height as f64 * shade_start_fraction) as usize;
        let shade_end = shade_start + shade_amount;

        for (i, y) in (2..bar_height + 2).enumerate() {
            if i >= shade_start && i < shade_end {
                print!("{}┃", cursor::Goto(self.dims.0, y));
            } else {
                print!("{}│", cursor::Goto(self.dims.0, y));
            }
        }
    }

    fn draw_children_of_root(&mut self) {
        trace!("draw_children_of_root()");
        let anchors = self.with_node(self.drawing_root, |n| n.children.clone()).unwrap();
        trace!("drawing children of root({}): {:?}",
               self.drawing_root,
               anchors);
        for child_id in anchors {
            let child_coords = self.with_node(child_id, |n| n.rooted_coords).unwrap();
            let child_color = self.with_node(child_id, |n| n.color.clone()).unwrap();
            let hide_stricken = self.with_node(self.drawing_root, |n| n.hide_stricken)
                .unwrap();
            self.draw_node(child_id,
                           "".to_owned(),
                           child_coords,
                           false,
                           hide_stricken,
                           child_color);
        }
    }

    // recursively draw node and children, returning how many have been drawn
    fn draw_node(&mut self,
                 node_id: NodeID,
                 prefix: String,
                 internal_coords: Coords,
                 last: bool,
                 hide_stricken: bool,
                 color: String)
                 -> usize {
        trace!("draw_node({})", node_id);
        let node = self.with_node(node_id, |n| n.clone()).unwrap();
        if node.stricken && hide_stricken {
            return 0;
        }

        // only actually print it if we're in-view
        if let Some(screen_coords) = self.internal_to_screen_xy(internal_coords) {
            let (x, y) = screen_coords;
            let mut buf = String::new();
            write!(&mut buf, "{}", cursor::Goto(x, y)).unwrap();
            write!(&mut buf, "{}", color).unwrap();
            if node.selected {
                write!(&mut buf, "{}", style::Invert).unwrap();
            }
            write!(&mut buf, "{}", prefix).unwrap();
            if prefix != "" {
                // only anchor will have blank prefix
                if last {
                    write!(&mut buf, "└─").unwrap();
                } else {
                    write!(&mut buf, "├─").unwrap();
                }
            }
            if node.stricken {
                write!(&mut buf, "☠").unwrap();
            } else if node.collapsed {
                write!(&mut buf, "⊞").unwrap();
            } else if node.hide_stricken {
                write!(&mut buf, "⚔").unwrap();
            } else {
                write!(&mut buf, " ").unwrap();
            }
            // keep color for selected & tree root Fg
            if !node.selected && prefix != "" {
                write!(&mut buf, "{}", color::Fg(color::Reset)).unwrap();
            }
            write!(&mut buf, "{}", node.content).unwrap();

            let max_width = (cmp::max(self.dims.0, 1 + x) - 1 - x) as usize;
            if false {
                // buf.chars().count() > max_width {
                let chars = buf.chars();
                // let oldlen = chars.clone().count();
                let mut truncated: String = chars.take(cmp::max(max_width, 1) - 1).collect();
                truncated.push('…');
                print!("{}", truncated);
            } else {
                print!("{}", buf);
            }
        }

        print!("{}", style::Reset);

        self.drawn_at.insert(node_id, internal_coords);
        for x in (internal_coords.0..(internal_coords.0 + 3 + prefix.len() as u16 +
                                      node.content.len() as u16))
            .rev() {
            trace!("inserting {:?} at {:?}", node_id, internal_coords);
            self.lookup.insert((x, internal_coords.1), node_id);
            if internal_coords.1 > self.lowest_drawn {
                self.lowest_drawn = internal_coords.1;
            }
        }
        let mut prefix = prefix;
        if last {
            prefix.push_str("   ");
        } else if prefix == "" {
            prefix.push_str(" ");
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
                let child_coords = (internal_coords.0, internal_coords.1 + drawn as u16);
                let child_drew = self.draw_node(child,
                                                prefix.clone(),
                                                child_coords,
                                                last,
                                                node.hide_stricken,
                                                color.clone());
                drawn += child_drew;
            }
        }
        drawn
    }

    fn draw_path(&self, internal_path: Vec<Coords>, start_dir: Dir, dest_dir: Dir) {
        let path: Vec<_> =
            internal_path.iter().filter_map(|&c| self.internal_to_screen_xy(c)).collect();
        trace!("draw_path({:?}, {:?}, {:?})", path, start_dir, dest_dir);
        print!("{}", random_fg_color());
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

    fn draw_header(&self) {
        trace!("draw_header()");
        let mut header_text = self.with_node(self.drawing_root, |node| node.content.clone())
            .unwrap();

        if self.should_auto_arrange() {
            header_text.push_str(" [auto-arrange] ");
        }

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


        if self.dims.0 > header_text.len() as u16 && self.dims.1 > 1 {
            let mut sep = format!("{}{}{}{}",
                                  cursor::Goto(0, 1),
                                  style::Invert,
                                  header_text,
                                  style::Reset);
            let text_len = header_text.chars().count();
            for _ in 0..(self.dims.0 as usize - text_len) {
                sep.push('█');
            }
            println!("{}", sep);
        }
    }

    fn path_from_node_to_point(&self, start: NodeID, to: Coords) -> (Vec<Coords>, (Dir, Dir)) {
        // TODO this is mostly copypasta from path_between_nodes, DRY
        trace!("getting path between node {} and point {:?}", start, to);
        let startbounds = self.bounds_for_lookup(start);
        if startbounds.is_none() {
            trace!("path_from_node_to_point exiting early, point not drawn");
            return (vec![], (Dir::R, Dir::R));
        }
        let (s1, s2) = startbounds.unwrap();
        let init = (self.path(s2, to), (Dir::R, Dir::R));
        let paths = vec![
            (self.path(s1, to), (Dir::L, Dir::R)),
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

    fn path_between_nodes(&self, start: NodeID, to: NodeID) -> (Vec<Coords>, (Dir, Dir)) {
        trace!("getting path between nodes {} and {}", start, to);
        let startbounds = self.bounds_for_lookup(start);
        let tobounds = self.bounds_for_lookup(to);
        if startbounds.is_none() || tobounds.is_none() {
            trace!("path_between_nodes exiting early, point not drawn");
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

    fn path(&self, start: Coords, dest: Coords) -> Vec<Coords> {
        trace!("path({:?}, {:?} (screen size: {} x {})",
               start,
               dest,
               self.dims.0,
               self.dims.1);
        fn perms(c: Coords) -> Vec<Coords> {
            vec![(c.0 + 1, c.1),
                 (cmp::max(c.0, 1) - 1, c.1),
                 (c.0, c.1 + 1),
                 // we ensure Y is >= 1, since Goto will panic otherwise
                 (c.0, cmp::max(c.1, 2) - 1)]
        }
        // maps from location to previous location
        let mut visited: HashMap<Coords, Coords> = HashMap::new();
        let mut pq = BinaryHeap::new();

        let mut cursor = start;
        trace!("starting draw");
        while cursor != dest {
            for neighbor in perms(cursor) {
                if (!(neighbor.0 >= self.dims.0) && !(neighbor.1 >= self.dims.1 + self.view_y) &&
                    !self.occupied(neighbor) || neighbor == dest) &&
                   !visited.contains_key(&neighbor) {
                    let c = std::u16::MAX - cost(neighbor, dest);
                    pq.push((c, neighbor));
                    visited.insert(neighbor, cursor);
                }
            }
            if let Some((_, coords)) = pq.pop() {
                cursor = coords;
            } else {
                trace!("no path, possible node overlap");
                return vec![];
            }
            // for tracing: show entire search path
            // self.draw_path(visited.clone().keys().map(|k| *k).collect());
        }
        trace!("done draw, starting backtrack");

        let mut back_cursor = dest;
        let mut path = vec![dest];
        while back_cursor != start {
            let prev = visited.get(&back_cursor).unwrap();
            path.push(*prev);
            back_cursor = *prev;
        }
        path.reverse();
        trace!("leaving path()");
        path
    }

    // correctness depends on invariant of the leftmost element being the
    // value in self.drawn_at
    fn bounds_for_lookup(&self, node_id: NodeID) -> Option<(Coords, Coords)> {
        if let Some(&left) = self.drawn_at.get(&node_id) {
            let mut rx = left.0;
            while let Some(&cursor) = self.lookup.get(&(rx + 1, left.1)) {
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

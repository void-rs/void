use std::{
    self,
    cmp::{max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    env,
    fmt::Write as FmtWrite,
    fs::{remove_file, rename, File, OpenOptions},
    io::{self, stdin, stdout, Error, ErrorKind, Read, Seek, SeekFrom, Stdout, Write},
    process,
};

use termion::{
    clear, color, cursor,
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
    style, terminal_size,
};

use rand::{self, Rng};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    cost, dateparse, distances, logging, now, plot, random_fg_color, re_matches, serialization,
    Action, Config, Coords, Dir, Node, NodeID, Pack, TagDB,
};

pub struct Screen {
    pub max_id: u64,
    pub nodes: HashMap<NodeID, Node>,
    pub arrows: Vec<(NodeID, NodeID)>,
    pub work_path: Option<String>,
    pub autosave_every: usize,
    pub config: Config,

    // screen dimensions as detected during the current draw() cycle
    pub dims: Coords,
    pub is_test: bool,

    // non-pub members are ephemeral
    drawing_root: NodeID,
    show_logs: bool,
    selected: Option<NodeID>,
    cut: Option<NodeID>,
    drawing_arrow: Option<NodeID>,
    lookup: HashMap<Coords, NodeID>,
    drawn_at: HashMap<NodeID, Coords>,
    dragging_from: Option<Coords>,
    dragging_to: Option<Coords>,
    stdout: Option<MouseTerminal<RawTerminal<AlternateScreen<Stdout>>>>,
    lowest_drawn: u16,
    // where we start drawing from
    view_y: u16,
    // when we drill down then pop up, we should go to last focus, stored here
    focus_stack: Vec<(NodeID, NodeID, u16)>,
    last_search: Option<(String, NodeID)>,

    // undo info
    undo_stack: Vec<NodeID>,
    // needs to be separate, as recursive deletion of nodes causes ordering issues
    undo_nodes: HashMap<NodeID, Node>,

    // nodes created specifically for rendering an augmented view
    ephemeral_nodes: HashMap<NodeID, Node>,
    // ephemeral max uses same keyspace, but resets on each frame,
    // and drops down from the top of the usize space. this is so
    // ephemeral and normal nodes SHOULD occupy the same keyspace
    // but be exclusive.
    ephemeral_max_id: u64,

    pub tag_db: TagDB,

    // timer for double clicks
    last_click_ms: u64,

    // grapheme calculation is expensive
    grapheme_cache: HashMap<NodeID, usize>,
}

impl Default for Screen {
    fn default() -> Screen {
        let mut root = Node::default();
        root.content = "home".to_owned();
        let mut screen = Screen {
            autosave_every: 25,
            config: Config::default(),
            arrows: vec![],
            selected: None,
            cut: None,
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
            dims: (1, 1),
            lowest_drawn: 0,
            view_y: 0,
            focus_stack: vec![],
            is_test: false,
            last_search: None,
            undo_stack: vec![],
            undo_nodes: HashMap::new(),
            ephemeral_nodes: HashMap::new(),
            ephemeral_max_id: std::u64::MAX,
            tag_db: TagDB::default(),
            last_click_ms: 0,
            grapheme_cache: HashMap::new(),
        };
        screen.nodes.insert(0, root);
        screen
    }
}

impl Screen {
    fn help(&mut self) {
        self.cleanup();
        println!("{}{}{}", cursor::Goto(1, 1), clear::All, self.config);
        self.start_raw_mode();
        if self.single_key_prompt("").is_err() {
            // likely here because of testing
        }
    }

    fn new_node_id(&mut self) -> NodeID {
        self.max_id += 1;
        assert!(self.max_id < self.ephemeral_max_id);
        self.max_id
    }

    fn new_node(&mut self) -> NodeID {
        let mut node = Node::default();
        let id = self.new_node_id();
        node.id = id;
        self.nodes.insert(id, node);
        id
    }

    pub fn with_node<B, F>(&self, k: NodeID, mut f: F) -> Option<B>
    where
        F: FnMut(&Node) -> B,
    {
        self.nodes.get(&k).map(|node| f(node))
    }

    fn with_node_mut<B, F>(&mut self, k: NodeID, mut f: F) -> Option<B>
    where
        F: FnMut(&mut Node) -> B,
    {
        self.nodes.get_mut(&k).map(|mut node| {
            node.meta.bump_mtime();
            f(&mut node)
        })
    }

    fn with_node_mut_no_meta<B, F>(&mut self, k: NodeID, mut f: F) -> Option<B>
    where
        F: FnMut(&mut Node) -> B,
    {
        self.nodes.get_mut(&k).map(|mut node| f(&mut node))
    }

    // return of false signals to the caller that we are done in this view
    pub fn handle_event(&mut self, evt: Event) -> bool {
        match self.config.map(evt) {
            Some(e) => match e {
                Action::LeftClick(x, y) => {
                    let internal_coords = self.screen_to_internal_xy((x, y));
                    self.click_screen(internal_coords)
                }
                Action::RightClick(..) => {
                    self.pop_focus();
                }
                Action::Release(x, y) => {
                    let internal_coords = self.screen_to_internal_xy((x, y));
                    self.release(internal_coords)
                }
                // Write character to selection
                Action::Char(c) if self.selected.is_some() => {
                    self.append(c);
                }
                Action::Char('/') => {
                    self.search_forward();
                }
                Action::Char('?') => {
                    self.search_backward();
                }
                Action::Char(c) => {
                    self.prefix_jump_to(c.to_string());
                }
                Action::Help => self.help(),
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
                Action::AutoArrange => self.toggle_auto_arrange(),
                Action::ToggleCollapsed => self.toggle_collapsed(),
                Action::Quit => return false,
                Action::Save => self.save(),
                Action::ToggleShowLogs => self.toggle_show_logs(),
                Action::EnterCmd => self.enter_cmd(),
                Action::FindTask => self.auto_task(),
                Action::YankPasteNode => self.cut_paste(),
                Action::RaiseSelected => self.raise_selected(),
                Action::LowerSelected => self.lower_selected(),
                Action::Search => self.search_forward(),
                Action::UndoDelete => self.undo_delete(),
                Action::SelectParent => self.select_parent(),
                Action::SelectNextSibling => self.select_next_sibling(),
                Action::SelectPrevSibling => self.select_prev_sibling(),
            },
            None => warn!("received unknown input"),
        }
        true
    }

    fn exists(&self, node_id: NodeID) -> bool {
        self.nodes.get(&node_id).is_some()
    }

    fn cut_paste(&mut self) {
        if let Some(selected_id) = self.selected {
            if let Some(cut) = self.cut.take() {
                self.reparent(cut, selected_id);
            } else {
                self.cut = Some(selected_id);
            }
        } else if let Some(cut) = self.cut.take() {
            let root = self.drawing_root;
            self.reparent(cut, root);
        }
    }

    fn reparent(&mut self, node_id: NodeID, parent_id: NodeID) {
        if !self.exists(node_id) || !self.exists(parent_id) {
            warn!("tried to reparent to deleted node");
            return;
        }
        if !self.is_parent(node_id, parent_id) {
            // reparent selected to parent_id
            // 1. remove from old parent's children
            // 2. add to new parent's children
            // 3. set parent_id pointer
            // 4. bust grapheme cache
            let old_parent = self.parent(node_id).unwrap();
            self.with_node_mut_no_meta(old_parent, |op| op.children.retain(|c| c != &node_id))
                .unwrap();
            self.with_node_mut_no_meta(parent_id, |np| np.children.push(node_id))
                .unwrap();
            self.with_node_mut_no_meta(node_id, |s| s.parent_id = parent_id)
                .unwrap();
            self.grapheme_cache.remove(&node_id);
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
            let mut incomplete_children: Vec<_> = node
                .children
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
            let prio = self
                .lineage(leaf)
                .iter()
                .filter_map(|&p| self.node_priority(p))
                .max()
                .unwrap_or(1);
            total_prio += prio;
            prio_pairs.push((prio, leaf));
        }

        if total_prio == 0 {
            // we're on a page with only zero priority tasks.
            // these are un-selectable automatically.
            return;
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
        self.zoom_select(choice);
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
                        .and_then(|n| n.get(1).unwrap().as_str().parse::<usize>().ok())
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

        let stdin: Box<dyn Read> = Box::new(stdin());
        print!(
            "{}{}{}{}",
            cursor::Goto(0, self.dims.1),
            style::Invert,
            clear::AfterCursor,
            prompt
        );
        self.flush();
        let res = stdin.keys().nth(0).unwrap();
        debug!("read prompt: {:?}", res);
        print!("{}", style::Reset);
        res
    }

    fn prompt(&mut self, prompt: &str) -> io::Result<Option<String>> {
        trace!("prompt({})", prompt);
        if self.is_test {
            return Err(Error::new(ErrorKind::Other, "can't prompt in test"));
        }

        let mut stdin: Box<dyn Read> = Box::new(stdin());
        print!(
            "{}{}{}{}{}",
            style::Invert,
            cursor::Goto(0, self.dims.1),
            clear::AfterCursor,
            prompt,
            cursor::Show
        );
        self.cleanup();
        let res = stdin.read_line();
        self.start_raw_mode();
        debug!("read prompt: {:?}", res);
        print!("{}", style::Reset);
        res
    }

    fn enter_cmd(&mut self) {
        trace!("enter_cmd()");
        if let Ok(Some(cmd)) = self.prompt("cmd: ") {
            debug!("received command {:?}", cmd);
        }
    }

    fn search_forward(&mut self) {
        self.search(SearchDirection::Forward)
    }

    fn search_backward(&mut self) {
        self.search(SearchDirection::Backward)
    }

    fn search(&mut self, direction: SearchDirection) {
        trace!("search()");
        let last_search_str = if let Some((ref last, _)) = self.last_search {
            format!(" [{}]: ", last)
        } else {
            "".to_owned()
        };
        let prompt = match direction {
            SearchDirection::Forward => format!("search{}:", last_search_str),
            SearchDirection::Backward => format!("search backwards{}:", last_search_str),
        };
        if let Ok(Some(mut query)) = self.prompt(&*prompt) {
            if query == "" {
                if let Some((ref last, _)) = self.last_search {
                    query = last.clone();
                } else {
                    self.last_search.take();
                    return;
                }
            } else {
                self.last_search.take();
            }

            let mut f = |n: &Node| n.content.find(&*query).map(|idx| (idx, n.id));
            let mut candidates = self.recursive_child_filter_map(self.drawing_root, &mut f);
            if candidates.is_empty() {
                return;
            }
            candidates.sort();
            let choice = if let Some((_, last_choice)) = self.last_search.take() {
                let idx = candidates
                    .iter()
                    .position(|&e| e.1 == last_choice)
                    .map(|i| match direction {
                        SearchDirection::Forward => i + 1,
                        SearchDirection::Backward => i + candidates.len() - 1,
                    })
                    .unwrap_or(0);
                candidates[idx % candidates.len()]
            } else {
                candidates[0]
            };

            self.last_search = Some((query.clone(), choice.1));
            self.zoom_select(choice.1);
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
            self.with_node(node_id, |n| n.content.starts_with(&*prefix))
                .unwrap()
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

        // clear the prompt
        print!("{}{}", cursor::Goto(1, self.dims.1), clear::AfterCursor);

        // print the hilighted char at each choice
        for (&c, &node_id) in &mapping {
            let &coords = self.drawn_at(node_id).unwrap();
            let (x, y) = self.internal_to_screen_xy(coords).unwrap();
            print!(
                "{}{}{}{}",
                cursor::Goto(x, y),
                style::Invert,
                c,
                style::Reset
            );
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
    where
        F: FnMut(NodeID) -> bool,
    {
        self.drawn_at
            .keys()
            .filter(|&node_id| self.node_is_visible(*node_id).unwrap())
            .filter(|&node_id| filter(*node_id))
            .cloned()
            .collect()
    }

    fn exec_selected(&mut self) {
        if self.is_test || self.selected.is_none() {
            // tests generate many randomly named nodes, so we don't
            // want to accidentally execute rm -rf /
            return;
        }
        let selected_id = self.selected.unwrap();

        let content_opt = self.with_node(selected_id, |n| n.content.clone());
        if content_opt.is_none() {
            error!("tried to exec deleted node");
            return;
        }
        let content = content_opt.unwrap();
        info!("executing command: {}", content);

        if content.is_empty() {
            error!("cannot execute empty command");
        } else if content.starts_with("txt:") {
            self.exec_text_editor(selected_id);
        } else if content.starts_with("http") {
            #[cfg(any(target_os = "macos",))]
            let default_open_cmd = "open";
            #[cfg(target_os = "linux")]
            let default_open_cmd = "xdg-open";
            #[cfg(target_os = "windows")]
            let default_open_cmd = "start";

            let browser = env::var("BROWSER").unwrap_or_else(|_| default_open_cmd.to_owned());
            let cmd = process::Command::new(browser).arg(&content).spawn();
            if cmd.is_err() {
                error!("command failed to start: {}", &content);
            }
        } else {
            let shell = env::var("SHELL").unwrap_or_else(|_| "bash".to_owned());
            let cmd = process::Command::new(shell).arg("-c").arg(&content).spawn();
            if cmd.is_err() {
                error!("command failed to start: {}", &content);
            }
        }
    }

    fn exec_text_editor(&mut self, node_id: NodeID) {
        let text = self
            .with_node(node_id, |n| n.free_text.clone())
            .unwrap()
            .unwrap_or_else(|| "".to_owned());

        let path = format!("/tmp/void_buffer.tmp.{}.md", process::id());
        debug!("trying to open {} in editor", path);

        // remove old tmp file
        if remove_file(&path).is_ok() {
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
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_owned());
        process::Command::new(editor)
            .arg(&path)
            .spawn()
            .expect("failed to open text editor")
            .wait()
            .unwrap();

        // read new data
        let mut data = vec![];

        {
            // File closed as it slides out of scope.
            let _ = File::open(&path).and_then(|mut f| f.read_to_end(&mut data));
        }

        let new_text = String::from_utf8(data).unwrap();

        let _ = remove_file(&path);

        // set node's saved text
        self.with_node_mut(node_id, |n| n.free_text = Some(new_text.clone()))
            .unwrap();

        // restore raw mode
        self.start_raw_mode();
    }

    pub fn arrange(&mut self) {
        trace!("arrange");
        let mut real_estate = Pack {
            children: None,
            top: 2,                // leave room for header
            left: 1,               // 1-indexed screen
            bottom: std::u16::MAX, // make this "bottomless" since we can paginate
            right: max(self.dims.0, 1) - 1,
            elem: None,
        };

        let nodes = self
            .with_node(self.drawing_root, |n| n.children.clone())
            .unwrap();
        let mut node_dims: Vec<(NodeID, Coords)> = nodes
            .into_iter()
            .map(|n| (n, self.drawable_subtree_dims(n).unwrap()))
            .collect();
        node_dims.sort_by_key(|&(_, (_, y))| y);
        node_dims.reverse();

        for (node_id, dims) in node_dims {
            // add some spacing around this tree to space out
            // placement a little bit
            let padded_dims = (dims.0 + 2, dims.1 + 2);
            if let Some((x, y)) = real_estate.insert(padded_dims) {
                self.with_node_mut_no_meta(node_id, |n| n.rooted_coords = (x, y))
                    .unwrap();
            }
        }
    }

    pub fn recursive_child_filter_map<F, B>(&self, node_id: NodeID, filter_map: &mut F) -> Vec<B>
    where
        F: FnMut(&Node) -> Option<B>,
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

    fn drawable_subtree_dims(&mut self, node_id: NodeID) -> Option<(u16, u16)> {
        if let Some(widths) = self.drawable_subtree_widths(node_id, 0) {
            let height = widths.len() as u16;
            let max_width = widths.into_iter().max().unwrap();
            Some((max_width, height))
        } else {
            None
        }
    }

    fn drawable_subtree_widths(&mut self, node_id: NodeID, depth: usize) -> Option<Vec<u16>> {
        let raw_node_opt = self.with_node(node_id, |n| n.clone());
        if let Some(raw_node) = raw_node_opt {
            let node = self.format_node(&raw_node);
            let width = 1 + (3 * depth as u16) + node.content.len() as u16;
            let mut ret = vec![width];
            let hide_stricken = self.with_node(node_id, |n| n.hide_stricken).unwrap();
            if !node.collapsed {
                for &child in &node.children {
                    let stricken = self.with_node(child, |c| c.stricken).unwrap();
                    if !(hide_stricken && stricken) {
                        // ASSUMES node.children are all valid
                        let mut child_widths =
                            self.drawable_subtree_widths(child, depth + 1).unwrap();
                        ret.append(&mut child_widths);
                    }
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
        lazy_static! {
            static ref RE_DATE: Regex = Regex::new(r"\[(\S+)\]").unwrap();
        }
        if let Some(selected_id) = self.selected {
            // nuke node if it's empty and has no children
            let deletable = self
                .with_node_mut_no_meta(selected_id, |n| {
                    n.selected = false;
                    n.content.is_empty() && n.children.is_empty()
                })
                .unwrap_or(false);
            if deletable {
                self.delete_selected(false);
                return None;
            }

            self.with_node_mut_no_meta(selected_id, |n| {
                // if parseable date, change date
                if let Some(date) = re_matches::<String>(&RE_DATE, &*n.content).get(0) {
                    if let Some(date) = dateparse(date.clone()) {
                        n.content = RE_DATE.replace(&*n.content, "").trim_end().to_owned();
                        if n.meta.finish_time.is_some() {
                            n.meta.finish_time = Some(date);
                        } else {
                            let now_in_s = now().as_secs();
                            let future_date = now_in_s + (now_in_s - date);
                            n.meta.due = Some(future_date);
                        }
                    }
                }
            });
        }
        self.selected.take()
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
        (
            coords.0,
            min(coords.1, std::u16::MAX - self.view_y) + self.view_y,
        )
    }

    fn coords_are_visible(&self, (_, y): Coords) -> bool {
        visible(self.view_y + 1, self.dims.1, y)
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
            self.unselect();
            if let Some(&node_id) = self.lookup(coords) {
                return self
                    .with_node_mut_no_meta(node_id, |node| {
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
        trace!("delete_recursive({})", node_id);
        if let Some(node) = self.nodes.remove(&node_id) {
            // clean up any arrow state
            self.arrows
                .retain(|&(ref from, ref to)| from != &node_id && to != &node_id);

            // remove from tag_db
            self.tag_db.remove(node_id);

            for child_id in &node.children {
                self.delete_recursive(*child_id);
            }

            self.undo_nodes.insert(node_id, node);
        }
    }

    fn delete_selected(&mut self, reselect: bool) {
        trace!("delete_selected()");
        if let Some(selected_id) = self.selected.take() {
            let (_, height) = self.drawable_subtree_dims(selected_id).unwrap();
            let coords = self.drawn_at.remove(&selected_id);
            // remove ref from parent
            if let Some(parent_id) = self.parent(selected_id) {
                trace!("deleting node {} from parent {}", selected_id, parent_id);
                self.with_node_mut_no_meta(parent_id, |p| p.children.retain(|c| c != &selected_id))
                    .unwrap();
            }
            // remove children
            self.delete_recursive(selected_id);
            if let Some((x, y)) = coords {
                if reselect {
                    self.click_select((x, y + height));
                }
            }
            self.undo_stack.push(selected_id);
        }
    }

    fn undo_delete(&mut self) {
        if let Some(node_id) = self.undo_stack.pop() {
            self.recursive_restore(node_id).unwrap();
            self.select_node(node_id);
        }
    }

    fn recursive_restore(&mut self, node_id: NodeID) -> Result<(), ()> {
        if let Some(node) = self.undo_nodes.remove(&node_id) {
            self.with_node_mut_no_meta(node.parent_id, |p| {
                if !p.children.contains(&node.id) {
                    p.children.push(node.id);
                }
            })
            .unwrap();
            let children = node.children.clone();
            self.nodes.insert(node_id, node);
            for &child in &children {
                self.recursive_restore(child)?;
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn should_auto_arrange(&self) -> bool {
        self.with_node(self.drawing_root, |n| n.auto_arrange)
            .unwrap()
    }

    fn toggle_auto_arrange(&mut self) {
        let root = self.drawing_root;
        self.with_node_mut_no_meta(root, |n| n.auto_arrange = !n.auto_arrange)
            .unwrap()
    }

    pub fn run(&mut self) {
        self.start_raw_mode();
        self.dims = terminal_size().unwrap();
        self.draw();
        let stdin = stdin();
        for (num_events, c) in stdin.events().enumerate() {
            let evt = c.unwrap();

            self.dims = terminal_size().unwrap();

            let should_break = !self.handle_event(evt);

            self.draw();

            if self.should_auto_arrange() {
                self.arrange();
                self.draw();
            }

            // if selected not visible, try to make it visible
            self.scroll_to_selected();

            // auto-save every 25 events to avoid larger data loss
            if num_events > 0 && num_events % self.autosave_every == 0 {
                self.save();
            }

            if should_break {
                self.cleanup();
                self.save();
                break;
            }
        }
        trace!("leaving stdin.events() loop");
        print!("{}{}", cursor::Goto(1, 1), clear::All);
    }

    fn toggle_collapsed(&mut self) {
        trace!("toggle_collapsed()");
        if let Some(selected_id) = self.selected {
            self.with_node_mut_no_meta(selected_id, |node| node.toggle_collapsed());
        }
    }

    fn toggle_show_logs(&mut self) {
        self.show_logs = !self.show_logs;
    }

    fn create_child(&mut self) {
        if let Some(mut selected_id) = self.selected {
            if self
                .with_node(selected_id, |n| n.content.is_empty())
                .unwrap()
            {
                // we may have hit tab after enter by accident,
                // so go forward a level by selecting the previous
                // child of the current parent
                let parent_id = self.parent(selected_id).unwrap();
                if parent_id == self.drawing_root {
                    // don't want to create a sibling of the drawing root
                    // because that's not underneath the drawing root
                    return;
                }

                let above = self
                    .with_node(parent_id, |parent| {
                        let idx = parent
                            .children
                            .iter()
                            .position(|&e| e == selected_id)
                            .unwrap();
                        parent.children[max(idx, 1) - 1]
                    })
                    .unwrap();

                self.select_node(above);
                selected_id = above;
            }
            let selected_id = selected_id;

            let node_id = self.new_node();
            self.with_node_mut_no_meta(node_id, |node| node.parent_id = selected_id);
            let added = self.with_node_mut_no_meta(selected_id, |selected| {
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
        if let Some(mut selected_id) = self.selected {
            if self
                .with_node(selected_id, |n| n.content.is_empty())
                .unwrap()
            {
                // we just hit enter twice, so go back a level
                let sel_parent = self.parent(selected_id).unwrap();
                if sel_parent == self.drawing_root {
                    // don't want to create a sibling of the drawing root
                    // because that's not underneath the drawing root
                    return;
                }
                self.select_node(sel_parent);
                selected_id = sel_parent;
            }
            let selected_id = selected_id;

            if let Some(parent_id) = self.parent(selected_id) {
                if parent_id == self.drawing_root {
                    self.create_child();
                    return;
                }
                let node_id = self.new_node();

                self.with_node_mut_no_meta(node_id, |node| node.parent_id = parent_id);
                let added = self.with_node_mut_no_meta(parent_id, |parent| {
                    // it's possible that selected_id has been deleted by now
                    // due to it being empty when we entered the function
                    // (double enter for going up a level)
                    let idx = parent
                        .children
                        .iter()
                        .position(|&e| e == selected_id)
                        .unwrap_or(0);
                    parent.children.insert(idx + 1, node_id);
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
            if self.dims.0 < 2 {
                from_x = Some(1);
                break;
            }
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
        self.with_node_mut_no_meta(node_id, |node| {
            node.rooted_coords = coords;
            node.parent_id = root;
        });
        self.with_node_mut_no_meta(root, |root| root.children.push(node_id));
        self.select_node(node_id);
    }

    fn backspace(&mut self) {
        trace!("backspace");
        if let Some(selected_id) = self.selected {
            if let Some(content) = self.with_node_mut(selected_id, |node| {
                let content = node.content.clone();
                let chars = content.chars();
                let oldlen = chars.clone().count();
                let truncated: String = chars.take(max(oldlen, 1) - 1).collect();
                node.content = truncated;
                node.content.clone()
            }) {
                self.grapheme_cache.remove(&selected_id);
                self.tag_db.reindex(selected_id, content);
            }
        }
    }

    fn append(&mut self, c: char) {
        trace!("append({})", c);
        if let Some(selected_id) = self.selected {
            if let Some(content) = self.with_node_mut(selected_id, |node| {
                node.content.push(c);
                node.content.clone()
            }) {
                self.grapheme_cache.remove(&selected_id);
                self.tag_db.reindex(selected_id, content);
            }
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
            ptr = self.parent(ptr).unwrap_or_else(|| {
                self.with_node(b, |n| error!("orphan node, have it cleaned up: {:?}", n));
                0
            });
        }
    }

    fn anchor(&self, node_id: NodeID) -> Result<NodeID, String> {
        if self.drawn_at(node_id).is_none() {
            return Err("node not drawn on this screen".to_owned());
        }

        // find the "root" just below self.drawing_root to mod
        // the rooted_coords for.
        let mut ptr = node_id;
        loop {
            let id = self.parent(ptr).ok_or("node has no parent")?;
            trace!(
                "anchor loop id: {} ptr: {} selected: {} root: {}",
                id,
                ptr,
                node_id,
                self.drawing_root
            );
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
                self.reparent(selected_id, new_parent);
            } else {
                // we're here because we released the drag
                // with the cursor over a child, so rather
                // than create a cycle, we move the subtree.
                let ptr = self.anchor(selected_id).unwrap();
                trace!("move selected 2");
                self.with_node_mut_no_meta(ptr, |root| {
                    let (ox, oy) = root.rooted_coords;
                    let nx = max(ox as i16 + dx, 1) as u16;
                    let ny = max(oy as i16 + dy, 1) as u16;
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
            self.with_node_mut_no_meta(old_parent, |op| op.children.retain(|c| c != &selected_id))
                .unwrap();
            let root = self.drawing_root;
            self.with_node_mut_no_meta(root, |dr| dr.children.push(selected_id))
                .unwrap();
            self.with_node_mut_no_meta(selected_id, |s| {
                s.rooted_coords = to;
                s.parent_id = root;
            })
            .unwrap();
        }
        trace!("leaving move_selected");
    }

    fn pop_focus(&mut self) {
        // bust grapheme cache on new view
        self.grapheme_cache.clear();
        self.unselect();
        let (root, selected, view_y) = self.focus_stack.pop().unwrap_or((0, 0, 0));
        self.drawing_root = root;
        self.view_y = view_y;
        self.select_node(selected);
    }

    fn select_parent(&mut self) {
        if let Some(selected_id) = self.selected {
            // If no parent, this should be a no-op.
            if let Some(parent_id) = self.parent(selected_id) {
                // If we're at a toplevel task (i.e., 0 is its parent ID), we don't want to
                // deselect it.
                if parent_id != 0 {
                    self.select_node(parent_id);
                }
            }
        }
    }

    fn select_next_sibling(&mut self) {
        self.select_neighbor(SearchDirection::Forward);
    }

    fn select_prev_sibling(&mut self) {
        self.select_neighbor(SearchDirection::Backward);
    }

    fn select_neighbor(&mut self, dir: SearchDirection) -> Option<NodeID> {
        use SearchDirection::*;
        let selected_id = self.selected?;
        let parent = self.nodes.get(&self.parent(selected_id)?)?;

        let selected_idx = parent.children.iter().position(|&id| id == selected_id)? as u64;
        let offset: isize = if dir == Forward { 1 } else { -1 };

        let neighbor_id = if let Some(&neighbor_id) = parent
            .children
            .get((selected_idx as isize + offset) as usize)
        {
            neighbor_id
        } else {
            let pos = if dir == Forward {
                0
            } else {
                parent.children.len() - 1
            };
            // Wrap around if there is no neighbor sibling. We know that
            // `parent.children` is nonempty because `selected_id` is one of them,
            // so the indexing is safe.
            parent.children[pos]
        };

        self.select_node(neighbor_id);

        None
    }

    fn drill_down(&mut self) {
        trace!("drill_down()");
        // bust grapheme cache on new view
        self.grapheme_cache.clear();
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
        let result = self.try_select(coords);
        self.dragging_from.take();
        self.dragging_to.take();
        result
    }

    fn scroll_up(&mut self) {
        self.view_y = max(self.view_y, self.dims.1 / 2) - self.dims.1 / 2;
        self.unselect();
    }

    fn scroll_down(&mut self) {
        if self.lowest_drawn > self.view_y + self.dims.1 {
            self.view_y = min(self.view_y + self.dims.1 / 2, self.lowest_drawn);
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
        if let Some(visible) = self.node_is_visible(node_id) {
            let &(_, y) = self.drawn_at(node_id).unwrap();
            if !visible {
                // move only if necessary
                self.view_y = max(y - 1, self.dims.1 / 2) - self.dims.1 / 2;
                self.draw();
                return true;
            }
        }
        false
    }

    fn zoom_select(&mut self, node_id: NodeID) {
        if !self.exists(node_id) {
            return;
        }
        // jump to highest view where node is visible
        let mut cursor = node_id;
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
        self.select_node(node_id);
        self.draw();
    }

    fn raise_selected(&mut self) {
        if let Some(selected_id) = self.selected {
            if !self.exists(selected_id) {
                warn!("tried to raise deleted node");
                return;
            }
            let parent_id = self.parent(selected_id).unwrap();
            if parent_id == self.drawing_root {
                // principle: don't modify things that are above the visible scope
                return;
            }
            self.with_node_mut_no_meta(parent_id, |parent| {
                let idx = parent
                    .children
                    .iter()
                    .position(|&e| e == selected_id)
                    .unwrap();
                let to = max(idx, 1) - 1;
                parent.children.swap(idx, to);
            });
        }
    }

    fn lower_selected(&mut self) {
        if let Some(selected_id) = self.selected {
            if !self.exists(selected_id) {
                warn!("tried to lower deleted node");
                return;
            }
            let parent_id = self.parent(selected_id).unwrap();
            if parent_id == self.drawing_root {
                // principle: don't modify things that are above the visible scope
                return;
            }
            self.with_node_mut_no_meta(parent_id, |parent| {
                let idx = parent
                    .children
                    .iter()
                    .position(|&e| e == selected_id)
                    .unwrap();
                let len = parent.children.len();
                if len > 1 {
                    let to = min(idx, len - 2) + 1;
                    parent.children.swap(idx, to);
                }
            });
        }
    }

    fn select_up(&mut self) {
        let view_y = self.view_y;
        let height = self.dims.1;
        self.select_relative(|(l1, _), (l2, _)| {
            let is_up = l1.1 > l2.1;
            let (diff_x, diff_y) = distances(l1, l2);
            if is_up {
                let visible = visible(view_y, height, l2.1);
                Some((!visible, diff_x, diff_y))
            } else {
                None
            }
        });
    }

    fn select_down(&mut self) {
        let view_y = self.view_y;
        let height = self.dims.1;
        self.select_relative(|(l1, _), (l2, _)| {
            let is_down = l1.1 < l2.1;
            let (diff_x, diff_y) = distances(l1, l2);
            if is_down {
                let visible = visible(view_y, height, l2.1);
                Some((!visible, diff_x, diff_y))
            } else {
                None
            }
        });
    }

    fn select_left(&mut self) {
        self.select_relative(|(l1, _), (_, r2)| {
            let is_left = l1.0 > r2.0;
            let (diff_x, diff_y) = distances(l1, r2);
            if is_left {
                Some((diff_y, diff_x))
            } else {
                None
            }
        })
    }

    fn select_right(&mut self) {
        self.select_relative(|(_, r1), (l2, _)| {
            let is_right = r1.0 < l2.0;
            let (diff_x, diff_y) = distances(r1, l2);
            if is_right {
                Some((diff_y, diff_x))
            } else {
                None
            }
        });
    }

    fn select_relative<F, O: Ord + Clone>(&mut self, filter_cost: F)
    where
        F: FnMut((Coords, Coords), (Coords, Coords)) -> Option<O>,
    {
        if let Some(node_id) = self.find_relative_node(filter_cost) {
            self.select_node(node_id);
        }
    }

    fn find_relative_node<F, O: Ord + Clone>(&mut self, mut filter_cost: F) -> Option<NodeID>
    where
        F: FnMut((Coords, Coords), (Coords, Coords)) -> Option<O>,
    {
        let default_coords = (self.dims.0 / 2, self.dims.1 / 2);
        let rel_def_coords = self.screen_to_internal_xy(default_coords);

        let cur = self
            .selected
            .and_then(|s| self.bounds_for_lookup(s))
            .unwrap_or((rel_def_coords, rel_def_coords));

        let mut node_costs = vec![];
        for node_id in self.drawn_at.keys() {
            if let Some(bounds) = self.bounds_for_lookup(*node_id) {
                if let Some(cost) = filter_cost(cur, bounds) {
                    node_costs.push((node_id, cost));
                }
            }
        }
        node_costs.sort_by_key(|&(_, ref cost)| cost.clone());
        node_costs.get(0).map(|&(&id, _)| id)
    }

    fn select_node(&mut self, node_id: NodeID) {
        trace!("select_node({})", node_id);
        self.unselect();
        if node_id != 0 {
            // it's possible that unselecting above actually caused
            // this node to be deleted, due to its parent (previous
            // selection) being empty.  To account for this, we need
            // to only set self.selected to node_id if the with_node
            // succeeds.
            if self
                .with_node_mut_no_meta(node_id, |node| node.selected = true)
                .is_some()
            {
                self.selected = Some(node_id);
            }
        }
    }

    fn click_screen(&mut self, coords: Coords) {
        trace!("click_screen({:?})", coords);
        if coords.0 > self.dims.0 || coords.1 > self.view_y + self.dims.1 {
            warn!("click way off-screen");
            return;
        }
        let old = self.selected;
        let new = self.try_select(coords);
        if old.is_none() && self.dragging_from.is_none() {
            self.create_anchor(coords);
        }

        // double click logic:
        // set click time, drill-down if we're below double click threshold
        let now = now();
        let now_ms = now.as_millis() as u64;

        let elapsed = now_ms - self.last_click_ms;

        if new.is_some() {
            self.last_click_ms = now_ms;
        }

        // if we double click, drill-down on it
        if new == old && elapsed <= 500 && new.is_some() {
            self.drill_down();
        }
    }

    fn release(&mut self, to: Coords) {
        trace!("release({:?})", to);
        if to.0 > self.dims.0 || to.1 > self.view_y + self.dims.1 {
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

        // no loops, no orphans
        debug!("testing that 0 is the ancestor of all nodes");
        for &node_id in self.nodes.keys() {
            assert!(self.is_parent(0, node_id));
        }

        debug!("testing that all arrows are existing nodes");
        // no arrows that don't exist
        for &(ref a, ref b) in &self.arrows {
            assert!(self.nodes.get(a).is_some());
            assert!(self.nodes.get(b).is_some());
        }
    }

    pub fn save(&self) {
        trace!("save()");
        self.assert_node_consistency();
        let data = serialization::serialize_screen(self);
        if let Some(ref path) = self.work_path {
            let mut tmp_path = path.clone();
            tmp_path.push_str(".tmp");
            if remove_file(&tmp_path).is_ok() {
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
            self.stdout = Some(MouseTerminal::from(
                AlternateScreen::from(stdout()).into_raw_mode().unwrap(),
            ));
        }
    }

    pub fn occupied(&self, coords: Coords) -> bool {
        self.lookup.contains_key(&coords)
    }

    pub fn add_or_remove_arrow(&mut self) {
        if self.drawing_arrow.is_none() {
            self.drawing_arrow = self.selected;
            return;
        }
        let from = self.drawing_arrow.take().unwrap();
        if let Some(arrow) = self.selected.map(|to| (from, to)) {
            let (from, to) = arrow;
            if self.nodes.get(&from).is_some() && self.nodes.get(&to).is_some() {
                let contains = self.arrows.iter().fold(false, |acc, &(ref nl1, ref nl2)| {
                    if nl1 == &from && nl2 == &to {
                        true
                    } else {
                        acc
                    }
                });
                if contains {
                    self.arrows.retain(|e| e != &arrow);
                } else {
                    self.arrows.push(arrow);
                }
            }
        }
    }

    // NB correctness depends on invariant of the leftmost element being the
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

    // *
    // *
    // *┌──┐
    // *│  Drawing Functionality
    // *└──────────────>
    // *
    // *

    pub fn draw(&mut self) {
        trace!("draw()");

        // let before = time::get_time();

        // clean up before a fresh drawing
        self.ephemeral_max_id = std::u64::MAX;
        self.ephemeral_nodes.clear();
        self.lookup.clear();
        self.drawn_at.clear();
        self.lowest_drawn = 0;
        print!("{}", clear::All);

        // print visible nodes
        self.draw_children_of_root();

        // TODO figure out why header doesn't get shown
        // when a root node is NOT drawn at 1,1
        // (this only happens when draw_header() is above
        // the call to draw_children_of_root()...
        self.draw_header();

        // print logs
        if self.show_logs && self.dims.0 > 4 && self.dims.1 > 7 {
            let mut sep = format!(
                "{}{}logs{}",
                cursor::Goto(0, self.dims.1 - 6),
                style::Invert,
                style::Reset
            );
            for _ in 0..self.dims.0 - 4 {
                sep.push('█');
            }
            println!("{}", sep);
            {
                let logs = logging::read_logs();
                for msg in logs.iter().rev() {
                    let line_width = min(msg.len(), self.dims.0 as usize);
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

        // let after = time::get_time();

        // debug!("draw time: {}", after - before);
    }

    fn draw_scrollbar(&self) {
        let bar_height = max(self.dims.1, 1) - 1;
        let normalized_lowest = f64::from(max(self.lowest_drawn, 1));
        let fraction_viewable = f64::from(self.dims.1) / normalized_lowest;
        let shade_start_fraction = f64::from(self.view_y) / normalized_lowest;

        let shade_amount = (f64::from(bar_height) * fraction_viewable) as usize;
        let shade_start = (f64::from(bar_height) * shade_start_fraction) as usize;
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
        let anchors = self
            .with_node(self.drawing_root, |n| n.children.clone())
            .unwrap();
        trace!(
            "drawing children of root({}): {:?}",
            self.drawing_root,
            anchors
        );
        for child_id in anchors {
            let child_coords = self.with_node(child_id, |n| n.rooted_coords).unwrap();
            let child_color = self.with_node(child_id, |n| n.color.clone()).unwrap();
            let hide_stricken = self
                .with_node(self.drawing_root, |n| n.hide_stricken)
                .unwrap();
            self.draw_node(
                child_id,
                "".to_owned(),
                child_coords,
                false,
                hide_stricken,
                child_color,
            );
        }
    }

    // recursively draw node and children, returning how many have been drawn
    fn draw_node(
        &mut self,
        node_id: NodeID,
        prefix: String,
        internal_coords: Coords,
        last: bool,
        hide_stricken: bool,
        color: String,
    ) -> usize {
        trace!("draw_node({})", node_id);
        let mut ephemeral = false;
        let raw_node = self
            .nodes
            .get(&node_id)
            .or_else(|| {
                ephemeral = true;
                self.ephemeral_nodes.get(&node_id)
            })
            .cloned()
            .unwrap();
        let node = if raw_node.selected {
            let mut formatted = self.format_node(&raw_node);
            formatted.content = raw_node.content;
            formatted
        } else {
            self.format_node(&raw_node)
        };
        if node.stricken && hide_stricken {
            return 0;
        }

        let reset = &*format!("{}", color::Fg(color::Reset));
        let mut pre_meta = String::new();
        let mut buf = String::new();

        // only actually print it if we're in-view
        if let Some((x, y)) = self.internal_to_screen_xy(internal_coords) {
            write!(pre_meta, "{}{}", cursor::Goto(x, y), color).unwrap();
            if node.selected {
                write!(&mut pre_meta, "{}", style::Invert).unwrap();
            }
            write!(&mut buf, "{}", pre_meta).unwrap();
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
            } else if node.free_text.is_some() {
                write!(&mut buf, "✏").unwrap();
            } else {
                write!(&mut buf, " ").unwrap();
            }
            // keep color for selected & tree root Fg
            if !node.selected && prefix != "" {
                write!(&mut buf, "{}", reset).unwrap();
            }

            write!(&mut buf, "{}", node.content).unwrap();

            let max_width = (max(self.dims.0, 1 + x) - 1 - x) as usize;
            let visible_graphemes =
                self.grapheme_cache
                    .get(&node.id)
                    .cloned()
                    .unwrap_or_else(|| {
                        let visible = buf.replace(reset, "").replace(&*pre_meta, "");
                        let vg = UnicodeSegmentation::graphemes(&*visible, true).count();
                        self.grapheme_cache.insert(node.id, vg);
                        vg
                    });
            if visible_graphemes > max_width {
                let buf_clone = buf.clone();
                let chars = buf_clone.chars();
                let width = chars.clone().count();
                let new_size = width - (visible_graphemes - max_width);
                buf = chars.take(new_size).collect();
                buf.push('…');
            }

            print!("{}{}", buf, style::Reset);
        }

        let visible_graphemes = self
            .grapheme_cache
            .get(&node.id)
            .cloned()
            .unwrap_or_else(|| {
                let visible = buf.replace(reset, "").replace(&*pre_meta, "");
                let vg = UnicodeSegmentation::graphemes(&*visible, true).count();
                self.grapheme_cache.insert(node.id, vg);
                vg
            });

        self.drawn_at.insert(node_id, internal_coords);
        for x in (internal_coords.0..(internal_coords.0 + visible_graphemes as u16)).rev() {
            trace!("inserting {:?} at {:?}", node_id, internal_coords);
            self.lookup.insert((x, internal_coords.1), node_id);
        }

        if internal_coords.1 > self.lowest_drawn {
            self.lowest_drawn = internal_coords.1;
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
            for (n, &child) in node.children.iter().enumerate() {
                let last = n + 1 == n_children;
                let child_coords = (internal_coords.0, internal_coords.1 + drawn as u16);
                let child_drew = self.draw_node(
                    child,
                    prefix.clone(),
                    child_coords,
                    last,
                    node.hide_stricken,
                    color.clone(),
                );
                drawn += child_drew;
            }
        }
        drawn
    }

    fn draw_path(&self, internal_path: Vec<Coords>, start_dir: Dir, dest_dir: Dir) {
        let path: Vec<_> = internal_path
            .iter()
            .filter_map(|&c| self.internal_to_screen_xy(c))
            .collect();
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
        let mut header_text = self
            .with_node(self.drawing_root, |node| node.content.clone())
            .unwrap();

        if self.should_auto_arrange() {
            header_text.push_str(" [auto-arrange] ");
        }

        let (plot, finished_today) = self.last_week_of_done_tasks();
        let plot_line = format!("│{}│({} today)", plot, finished_today);

        header_text.push_str(&*plot_line);

        if self.dims.0 > header_text.len() as u16 && self.dims.1 > 1 {
            let mut sep = format!(
                "{}{}{}{}",
                cursor::Goto(0, 1),
                style::Invert,
                header_text,
                style::Reset
            );
            let text_len = header_text.chars().count();
            for _ in 0..(max(self.dims.0 as usize, text_len) - text_len) {
                sep.push('█');
            }
            println!("{}", sep);
        }
    }

    fn path_from_node_to_point(&self, start: NodeID, to: Coords) -> (Vec<Coords>, (Dir, Dir)) {
        trace!("getting path between node {} and point {:?}", start, to);
        let startbounds = self.bounds_for_lookup(start);
        if startbounds.is_none() {
            trace!("path_from_node_to_point exiting early, point not drawn");
            return (vec![], (Dir::R, Dir::R));
        }
        let (s1, s2) = startbounds.unwrap();

        self.path_with_directions(
            &[(s1, Dir::L), (s2, Dir::R)],
            &[(to, Dir::R)],
        )
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

        self.path_with_directions(
            &[(s1, Dir::L), (s2, Dir::R)],
            &[(t1, Dir::L), (t2, Dir::R)],
        )
    }

    fn path_with_directions(
        &self,
        starts: &[(Coords, Dir)],
        dests: &[(Coords, Dir)]
    ) -> (Vec<Coords>, (Dir, Dir)) {
        let dests_no_dir: Vec<Coords> = dests.iter()
            .map(|(coords, _)| *coords)
            .collect();
        let path = self.path(starts, &dests_no_dir);
        let last_node = if let Some(n) = path.last() { n } else {
            return (path, (Dir::L, Dir::R));
        };
        let first_node = path.first().unwrap();
        let last_dir = dests.iter()
            .filter(|(dest, _)| dest == last_node)
            .map(|(_, dir)| dir)
            .next()
            .unwrap();
        let first_dir = starts.iter()
            .filter(|(start, _)| start == first_node)
            .map(|(_, dir)| dir)
            .next()
            .unwrap();

        (path, (*first_dir, *last_dir))
    }

    fn path(&self, starts: &[(Coords, Dir)], dests: &[Coords]) -> Vec<Coords> {
        trace!(
            "path({:?}, {:?} (screen size: {} x {})",
            starts,
            dests,
            self.dims.0,
            self.dims.1
        );
        fn perms(c: Coords) -> Vec<Coords> {
            vec![
                (c.0 + 1, c.1),
                (max(c.0, 1) - 1, c.1),
                (c.0, c.1 + 1),
                // we ensure Y is >= 1, since Goto will panic otherwise
                (c.0, max(c.1, 2) - 1),
            ]
        }
        let heuristic = |from: Coords|
            dests.iter()
                .map(|dest| cost(from, *dest))
                .min()
                .unwrap_or(std::u16::MAX);
        // maps from location to previous location
        let mut visited: HashMap<Coords, (Coords, u16)> = HashMap::new();

        // priority queue of nodes to explore, initially populated w/ starting locs
        // tuple is (priority, coords, last_direction, cost)
        let mut pq: BinaryHeap<_> = starts.into_iter()
            .map(|(point, dir)| (
                std::u16::MAX - heuristic(*point),
                *point,
                match dir {
                    Dir::L => -1,
                    Dir::R => 1,
                },
                0
            ))
            .collect();

        let (_, mut cursor, mut cursor_last_direction, mut cursor_cost) =
            pq.pop().expect("path() called without any starting point");
        trace!("starting draw");
        while !dests.contains(&cursor) {
            for neighbor in perms(cursor) {
                // direction is -2, -1, 1, or 2
                let direction = (neighbor.0 as i32) - (cursor.0 as i32)
                         + 2 * ((neighbor.1 as i32) - (cursor.1 as i32));

                let move_cost = if cursor_last_direction == direction {
                    1 // We're moving in the same direction as before: free
                } else {
                    2 // We changed direction, which is discouraged to arrows simple
                };

                let turn_into_dest_cost = if
                    (direction == -2 || direction == 2) &&
                    heuristic(neighbor) == 0
                {
                    // When we arrive at dest, it's good to be traveling in the direction
                    // that the carrot will be pointing.  e.g.
                    //
                    // Bad: ──┐       Good:─┐
                    //        │             │
                    //        >dest         └─>dest
                    5
                } else {
                    0
                };

                // Total cost to get to this point
                let total_cost = move_cost + turn_into_dest_cost + cursor_cost;

                if (neighbor.0 < self.dims.0
                    && neighbor.1 < self.dims.1 + self.view_y
                    && !self.occupied(neighbor)
                    || dests.contains(&neighbor))
                    && visited.get(&neighbor) // Only if we found...
                        .map(|(_, old_cost)| *old_cost > total_cost) // a cheaper route...
                        .unwrap_or(true) // or the first route
                {
                    let priority = std::u16::MAX
                        - heuristic(neighbor)
                        - total_cost;

                    pq.push((priority, neighbor, direction, total_cost));
                    visited.insert(neighbor, (cursor, total_cost));
                }
            }
            if let Some((_, coords, last_direction, cost)) = pq.pop() {
                cursor = coords;
                cursor_cost = cost;
                cursor_last_direction = last_direction;
            } else {
                trace!("no path, possible node overlap");
                return vec![];
            }
            // for tracing: show entire search path
            // self.draw_path(visited.clone().keys().map(|k| *k).collect());
        }
        trace!("done draw, starting backtrack");

        let mut back_cursor = cursor;
        let mut path = vec![cursor];
        while !starts.iter().any(|(start, _)| *start == back_cursor) {
            let (prev, _) = visited[&back_cursor];
            path.push(prev);
            back_cursor = prev;
        }
        path.reverse();
        trace!("leaving path()");
        path
    }

    fn last_week_of_done_tasks(&self) -> (String, usize) {
        let now = now().as_secs();
        let day_in_sec = 60 * 60 * 24;
        let last_week = now - (day_in_sec * 7);
        let tasks_finished_in_last_week = self.recursive_child_filter_map(0, &mut |n: &Node| {
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
        let finished_today = counts_clone[&today_normalized];
        let week_line: Vec<i64> = counts.into_iter().map(|(_, v)| v).collect();
        let plot = plot::plot_sparkline(week_line);
        (plot, finished_today as usize)
    }

    fn format_node(&mut self, raw_node: &Node) -> Node {
        lazy_static! {
            //// general subtree population and modification
            // limit shows the top N results.
            static ref RE_LIMIT: Regex = Regex::new(r"#limit=(\d+)").unwrap();
            static ref RE_TAGGED: Regex = Regex::new(r"#tagged=(\S+)").unwrap();
            static ref RE_REV: Regex = Regex::new(r"#rev\b").unwrap();
            static ref RE_DONE: Regex = Regex::new(r"#done\b").unwrap();
            static ref RE_OPEN: Regex = Regex::new(r"#open\b").unwrap();
            // since defaults to last week
            static ref RE_SINCE: Regex = Regex::new(r"#since=(\S+)").unwrap();
            // until defaults until now
            static ref RE_UNTIL: Regex = Regex::new(r"#until=(\S+)").unwrap();

            //// plot specific
            // plot can be {new,done}
            static ref RE_PLOT: Regex = Regex::new(r"#plot=(\S+)").unwrap();
            // n is the number of buckets
            static ref RE_N: Regex = Regex::new(r"#n=(\d+)").unwrap();
        }

        // TODO detect and avoid cycles
        // NB avoid cycles
        let mut node = raw_node.clone();

        // for tagged queries, AND queries together
        let mut tagged_children: Option<HashSet<NodeID>> = None;
        for tag in &re_matches::<String>(&RE_TAGGED, &*node.content) {
            let children = self.tag_db.tag_to_nodes(tag);
            if let Some(children_acc) = tagged_children {
                let children_new = children.into_iter().collect();
                let intersection = children_acc.intersection(&children_new);
                tagged_children = Some(intersection.cloned().collect());
            } else {
                tagged_children = Some(children.into_iter().collect());
            }
        }
        let queried_nodes = tagged_children
            .map(|tc| tc.into_iter().collect())
            .unwrap_or_else(|| vec![]);

        let mut since_opt = None;
        let mut until_opt = None;
        if RE_DONE.is_match(&*node.content) {
            for child in node.children.clone() {
                let done = self.with_node(child, |c| c.stricken).unwrap();
                if !done {
                    node.children.retain(|&c| c != child);
                }
            }
        }
        if RE_OPEN.is_match(&*node.content) {
            for child in node.children.clone() {
                let open = self.with_node(child, |c| !c.stricken).unwrap();
                if !open {
                    node.children.retain(|&c| c != child);
                }
            }
        }
        if let Some(since) = re_matches::<String>(&RE_SINCE, &*node.content).get(0) {
            since_opt = dateparse(since.clone());
            if let Some(cutoff) = since_opt {
                let mut new = vec![];
                for &c in &node.children {
                    let valid = self
                        .with_node(c, |c| c.meta.mtime >= cutoff)
                        .unwrap_or(false);
                    if valid {
                        new.push(c);
                    }
                }
                node.children = new;
            }
        }
        if let Some(until) = re_matches::<String>(&RE_UNTIL, &*node.content).get(0) {
            until_opt = dateparse(until.clone());
            if let Some(cutoff) = until_opt {
                let mut new = vec![];
                for &c in &node.children {
                    let valid = self
                        .with_node(c, |c| c.meta.mtime <= cutoff)
                        .unwrap_or(false);
                    if valid {
                        new.push(c);
                    }
                }
                node.children = new;
            }
        }
        if RE_REV.is_match(&*node.content) {
            node.children = node.children.into_iter().rev().collect();
        }
        if let Some(&limit) = re_matches(&RE_LIMIT, &*node.content).get(0) {
            node.children.truncate(limit);
        }

        let re_n = re_matches::<usize>(&RE_N, &*node.content);
        let n_opt = re_n.get(0);
        if let Some(plot) = re_matches::<String>(&RE_PLOT, &*node.content).get(0) {
            let now = now().as_secs();
            let buckets = n_opt.cloned().unwrap_or(7);
            let since = since_opt.unwrap_or_else(|| now - 60 * 60 * 24 * 7);
            let until = until_opt.unwrap_or_else(|| now);

            node.content = match plot.as_str() {
                "done" => self.plot(queried_nodes, PlotType::Done, buckets, since, until),
                "new" => self.plot(queried_nodes, PlotType::New, buckets, since, until),
                _ => node.content,
            };
        }
        node
    }

    fn plot(
        &self,
        queried_nodes: Vec<NodeID>,
        kind: PlotType,
        buckets: usize,
        since: u64,
        until: u64,
    ) -> String {
        let mut nodes = vec![];
        for &c in &queried_nodes {
            let mut new = self.recursive_child_filter_map(c, &mut |n: &Node| match kind {
                PlotType::Done => {
                    if let Some(ft) = n.meta.finish_time {
                        if ft >= since {
                            return Some(ft as i64);
                        }
                    }
                    None
                }
                PlotType::New => {
                    if n.meta.ctime >= since {
                        Some(n.meta.ctime as i64)
                    } else {
                        None
                    }
                }
            });
            nodes.append(&mut new);
        }
        let plot = plot::bounded_count_sparkline(nodes, since as i64, until as i64, buckets);
        format!("|{}|", plot)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum SearchDirection {
    Forward,
    Backward,
}

enum PlotType {
    New,
    Done,
}

fn visible(view_y: u16, height: u16, y: u16) -> bool {
    y > view_y && y < view_y + height
}

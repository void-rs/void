use std;
use std::cmp;
use std::fs::{File, rename, remove_file};
use std::collections::BTreeMap;
use std::io::{Write, Stdout, stdout, stdin};

use termion::{terminal_size, cursor, style, clear};
use termion::color;
use termion::event::{Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

use mindmap::{NodeID, Coords, Node, serialization, random_color, PrioQueue};
use logging;

pub struct Screen {
    pub max_id: u64,
    pub drawing_root: NodeID,
    pub nodes: BTreeMap<NodeID, Node>,
    pub arrows: Vec<(NodeID, NodeID)>,
    pub work_path: Option<String>,
    last_selected: Option<NodeID>,
    drawing_arrow: Option<NodeID>,
    dragging_from: Option<Coords>,
    lookup: BTreeMap<Coords, NodeID>,
    drawn_at: BTreeMap<NodeID, Coords>,
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
            nodes: BTreeMap::new(),
            lookup: BTreeMap::new(),
            drawn_at: BTreeMap::new(),
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
                    Ctrl('f') => self.toggle_hide_stricken(),
                    Ctrl('x') => self.toggle_stricken(),
                    Ctrl('a') => self.draw_arrow(),
                    Ctrl('o') => self.drill_down(),
                    Ctrl('t') => self.pop_focus(),
                    Alt('\u{1b}') | Ctrl('c') | Ctrl('d') => return false,
                    Ctrl('s') | Ctrl('w') => self.save(),
                    Up => self.select_up(),
                    Down => self.select_down(),
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

    // recursively draw node and children, returning how many have been drawn
    fn draw_node(&mut self,
                 node_id: NodeID,
                 prefix: String,
                 coords: Coords,
                 last: bool,
                 hide_stricken: bool)
                 -> usize {
        // debug!("drawing node {}", node_id);
        let (x, y) = coords;
        let node = self.with_node(node_id, |n| n.clone()).unwrap();
        if node.stricken && hide_stricken {
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
        for x in (x..(x + 3 + prefix.len() as u16 + node.content.len() as u16)).rev() {
            // debug!("inserting {:?} at {:?}", node_id, (x, y));
            self.lookup.insert((x, y), node_id);
            self.drawn_at.insert(node_id, (x, y));
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

    fn draw_from_root(&mut self) {
        self.lookup.clear();
        self.drawn_at.clear();
        let anchors = self.with_node(self.drawing_root, |n| n.children.clone()).unwrap();
        for child_id in anchors {
            let coords = self.with_node(child_id, |n| n.rooted_coords).unwrap();
            let hide_stricken = self.with_node(child_id, |n| n.hide_stricken).unwrap();
            self.draw_node(child_id, "".to_owned(), coords, false, hide_stricken);
        }
    }

    fn draw(&mut self) {
        print!("{}", clear::All);

        let (width, bottom) = terminal_size().unwrap();

        // print header
        let header_text = self.with_node(self.drawing_root, |node| node.content.clone())
            .unwrap();

        if width > header_text.len() as u16 && bottom > 1 {
            let mut sep = format!("{}{}{}{}",
                                  cursor::Goto(0, 1),
                                  style::Invert,
                                  header_text,
                                  style::Reset);
            for _ in 0..width as usize - header_text.len() {
                sep.push('█');
            }
            println!("{}", sep);
        }

        // print visible nodes
        self.draw_from_root();

        // print logs
        if width > 4 && bottom > 7 {
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
            let path = self.path_between_nodes(*from, *to);
            self.draw_path(path);
        }

        print!("{}", cursor::Hide);
        if let Some(mut s) = self.stdout.take() {
            s.flush().unwrap();
            self.stdout = Some(s);
        }
    }

    fn pop_selected(&mut self) -> Option<NodeID> {
        if self.dragging_from.is_none() {
            if let Some(selected_id) = self.last_selected.take() {
                debug!("popping selected");
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
        debug!("trying_select({:?}", coords);
        if self.dragging_from.is_none() {
            if let Some(&node_id) = self.lookup.get(&coords) {
                return self.with_node_mut(node_id, |mut node| {
                        debug!("selected node {} at {:?}", node_id, coords);
                        node.selected = true;
                        node_id
                    })
                    .and_then(|id| {
                        self.last_selected = Some(node_id);
                        self.dragging_from = Some(coords);
                        Some(id)
                    })
                    .or_else(|| {
                        debug!("found no node at {:?}", coords);
                        None
                    });
            }
        }
        debug!("selected no node at {:?}", coords);
        // debug!("lookup is {:?}", self.lookup);
        None
    }

    fn toggle_stricken(&mut self) {
        if let Some(selected_id) = self.last_selected {
            debug!("toggle stricken");
            self.with_node_mut(selected_id, |node| node.toggle_stricken());
        }
    }

    fn toggle_hide_stricken(&mut self) {
        if let Some(selected_id) = self.last_selected {
            debug!("toggle hide stricken");
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
            let parent_id = self.with_node(selected_id, |n| n.parent_id).unwrap();
            debug!("deleting selected");
            self.with_node_mut(parent_id, |p| p.children.retain(|c| c != &selected_id));

            // remove children
            self.delete_recursive(selected_id);
            if let Some(c) = coords {
                self.click_select(c);
            }
        }
    }

    fn create_child(&mut self) {
        if let Some(selected_id) = self.last_selected {
            let node_id = self.new_node();
            debug!("creating child attributes");
            let added = self.with_node_mut(selected_id, |selected| {
                selected.children.push(node_id);
            });
            if added.is_some() {
                self.with_node_mut(node_id, |node| node.parent_id = selected_id);
                self.select_node(node_id);
            } else {
                self.delete_recursive(node_id);
            }
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
            let should_break = !self.handle_event(evt);
            self.draw();
            if should_break {
                self.cleanup_and_save();
                break;
            }
        }
    }

    fn toggle_collapsed(&mut self) {
        if let Some(selected_id) = self.last_selected {
            debug!("collapsed toggle");
            self.with_node_mut(selected_id, |node| node.toggle_collapsed());
        }
    }

    fn create_anchor(&mut self, coords: Coords) {
        let root = self.drawing_root;
        let node_id = self.new_node();
        debug!("setting node parent and rooted_coords");
        self.with_node_mut(node_id, |node| {
            node.rooted_coords = coords;
            node.parent_id = root;
        });
        debug!("creating anchor");
        self.with_node_mut(root, |root| root.children.push(node_id));
    }

    fn backspace(&mut self) {
        if let Some(selected_id) = self.last_selected {
            debug!("backspace");
            self.with_node_mut(selected_id, |node| {
                let newlen = std::cmp::max(node.content.len(), 1) - 1;
                node.content = node.content.clone()[..newlen].to_owned();
            });
        }
    }

    fn append(&mut self, c: char) {
        if let Some(selected_id) = self.last_selected {
            debug!("append");
            self.with_node_mut(selected_id, |node| {
                node.content.push(c);
            });
        }
    }

    // TODO after NodeID refactor add support for moving
    // children to other trees here. this will have the nice
    // effect of no longer having weird overlapping nodes
    // as often (ever?)
    fn move_selected(&mut self, from: Coords, to: Coords) {
        let dx = to.0 as i16 - from.0 as i16;
        let dy = to.1 as i16 - from.1 as i16;

        if let Some(selected_id) = self.last_selected {
            // find the "root" just below self.drawing_root to mod
            // the rooted_coords for.
            let mut ptr = selected_id;
            loop {
                let id = self.with_node(ptr, |node| node.parent_id).unwrap();
                debug!("move selected 1, id: {} ptr: {} selected: {} root: {}",
                       id,
                       ptr,
                       selected_id,
                       self.drawing_root);
                if id != self.drawing_root {
                    ptr = id;
                } else {
                    break;
                }
            }

            debug!("move selected 2");
            self.with_node_mut(ptr, |mut root| {
                    let coords = root.rooted_coords;
                    let nx = cmp::max(coords.0 as i16 + dx, 1) as u16;
                    let ny = cmp::max(coords.1 as i16 + dy, 1) as u16;
                    root.rooted_coords = (nx, ny);
                })
                .unwrap();
        }
    }

    fn pop_focus(&mut self) {
        let parent_id = self.with_node(self.drawing_root, |root| root.parent_id).unwrap();
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
        debug!("click_select({:?})", coords);
        self.pop_selected();
        let result = self.try_select((coords.0, coords.1));
        self.dragging_from.take();
        result
    }

    fn select_up(&mut self) {
        if let Some(selected_id) = self.last_selected {
            if let Some(&coords) = self.drawn_at.get(&selected_id) {
                // to prevent selection fall-off, click old coords
                // if nothing is selected above this node
                self.click_select((coords.0, coords.1 - 1))
                    .or_else(|| self.click_select(coords));
            }
        }
    }

    fn select_down(&mut self) {
        if let Some(selected_id) = self.last_selected {
            if let Some(&coords) = self.drawn_at.get(&selected_id) {
                // to prevent selection fall-off, click old coords
                // if nothing is selected below this node
                self.click_select((coords.0, coords.1 + 1))
                    .or_else(|| self.click_select(coords));
            }
        }
    }

    fn select_node(&mut self, node_id: NodeID) {
        self.pop_selected();
        debug!("select_node");
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
                debug!("removed stale tmp file");
            }
            let mut f = File::create(&tmp_path).unwrap();
            f.write_all(&*data).unwrap();
            f.sync_all().unwrap();
            rename(tmp_path, path).unwrap();
            info!("saved work to {}", path);
        }
    }

    fn cleanup_and_save(&mut self) {
        let (_, bottom) = terminal_size().unwrap();
        print!("{}", cursor::Goto(0, bottom));
        println!("{}", cursor::Show);
        self.stdout.take().unwrap().flush().unwrap();
        self.save();
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
        fn cost(c1: Coords, c2: Coords) -> u16 {
            let xcost = cmp::max(c1.0, c2.0) - cmp::min(c1.0, c2.0);
            let ycost = cmp::max(c1.1, c2.1) - cmp::min(c1.1, c2.1);
            xcost + ycost
        }
        fn perms(c: Coords) -> Vec<Coords> {
            vec![(c.0 + 1, c.1),
                 (cmp::max(c.0, 1) - 1, c.1),
                 (c.0, c.1 + 1),
                 (c.0, cmp::max(c.1, 1) - 1)]
        }
        // maps from location to previous location
        let mut visited: BTreeMap<Coords, Coords> = BTreeMap::new();
        let mut pq = PrioQueue::default();

        // TODO start with dest, go backwards, that way we don't need to reverse
        // draw tree greedily
        let mut cursor = start;
        while cursor != dest {
            for neighbor in perms(cursor) {
                if (!self.occupied(neighbor) || neighbor == dest) &&
                   !visited.contains_key(&neighbor) {
                    let c = cost(neighbor, dest);
                    pq.insert(c, neighbor);
                    visited.insert(neighbor, cursor);
                }
            }
            cursor = pq.pop().unwrap();
            // for debugging: show entire search path
            // self.draw_path(visited.clone().keys().map(|k| *k).collect());
        }

        let mut back_cursor = dest;
        let mut path = vec![dest];
        while back_cursor != start {
            let prev = visited.get(&back_cursor).unwrap();
            path.push(*prev);
            back_cursor = *prev;
        }
        path.reverse();
        path
    }

    fn draw_path(&self, path: Vec<Coords>) {
        print!("{}", random_color());
        if path.len() == 1 {
            print!("{} ↺", cursor::Goto(path[0].0, path[0].1))
        } else if path.len() > 1 {
            let first = if path[1].1 > path[0].1 {
                '┐'
            } else if path[1].1 < path[0].1 {
                '┘'
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
            let end_char = if path[path.len() - 2].0 < end_x {
                '>'
            } else {
                '<'
            };
            print!("{}{}", cursor::Goto(end_x, end_y), end_char);
        }
        print!("{}", color::Fg(color::Reset));
    }

    fn path_between_nodes(&self, start: NodeID, to: NodeID) -> Vec<Coords> {
        // debug!("getting path between {} and {}", start, to);
        let startbounds = self.bounds_for_lookup(start);
        let tobounds = self.bounds_for_lookup(to);
        if startbounds.is_none() || tobounds.is_none() {
            debug!("path_between_nodes exiting early, point not drawn");
            return vec![];
        }
        let (_, s2) = startbounds.unwrap();
        let (_, t2) = tobounds.unwrap();

        let init = self.path(s2, t2);
        let paths = vec![
            init.clone()
            //self.path(s1, t2),
            //self.path(s2, t1),
            //self.path(s1, t1),
        ];
        paths.into_iter()
            .fold(init, |short, path| {
                if path.len() < short.len() {
                    path
                } else {
                    short
                }
            })
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

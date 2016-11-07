use std;
use std::cmp;
use std::fs::{File, rename, remove_file};
use std::collections::BTreeMap;
use std::io::{Write, Stdout, stdout, stdin};

use termion::{terminal_size, cursor, style};
use termion::color;
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

use mindmap::{NodeID, Coords, Node, serialization, random_color, PrioQueue};
use logging;

pub struct Screen {
    pub work_path: Option<String>,
    stdout: Option<MouseTerminal<RawTerminal<Stdout>>>,
    dragging_from: Option<Coords>,

    // REFACTOR CHANGE
    pub arrows: Vec<(NodeID, NodeID)>,
    last_selected: Option<NodeID>,
    drawing_arrow: Option<NodeID>,

    // REFACTOR IN
    pub max_id: u64,
    pub drawing_root: NodeID,
    pub nodes: BTreeMap<NodeID, Node>,
    pub lookup: BTreeMap<Coords, NodeID>,
    pub drawn_at: BTreeMap<NodeID, Coords>,
}

impl Default for Screen {
    fn default() -> Screen {
        let mut root = Node::default();
        root.content = "home".to_owned();
        let mut screen = Screen {
            // REFACTOR CHANGE
            arrows: vec![],
            last_selected: None,
            drawing_arrow: None,

            // REFACTOR IN
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

// refactor plan
// 1. populate nodes, lookup, drawn on draw()
// 2. populate arrows_new on serialization by modifying insert arrow

impl Screen {
    fn new_node(&mut self) -> NodeID {
        let mut node = Node::default();
        let id = self.max_id;
        self.max_id += 1;
        node.id = id;
        self.nodes.insert(id, node);
        id
    }

    fn with_node<B, F>(&self, k: NodeID, mut f: F) -> Option<B>
        where F: FnMut(&Node) -> B
    {
        self.nodes.get(&k).map(|v| f(v))
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
        match evt {
            Event::Key(Key::Char('\n')) => self.toggle_collapsed(),
            Event::Key(Key::Char('\t')) => self.create_child(),
            Event::Key(Key::Delete) => self.delete_selected(),
            Event::Key(Key::Ctrl('f')) => self.toggle_hide_stricken(),
            Event::Key(Key::Ctrl('x')) => self.toggle_stricken(),
            Event::Key(Key::Ctrl('a')) => self.draw_arrow(),
            Event::Key(Key::Alt('\u{1b}')) |
            Event::Key(Key::Ctrl('c')) |
            Event::Key(Key::Ctrl('d')) => return false,
            Event::Key(Key::Ctrl('s')) |
            Event::Key(Key::Ctrl('w')) => self.save(),
            Event::Key(Key::Up) => self.select_up(),
            Event::Key(Key::Down) => self.select_down(),
            Event::Key(Key::Backspace) => self.backspace(),
            Event::Key(Key::Char(c)) => {
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

    fn draw_tree(&mut self) {
        // print trees that are children of drawing_root
        // TODO
        //
        // let mut mappings = vec![];
        // for (node_id, node) in &self.anchors {
        // let (_, mut mapped) = anchor.borrow()
        // .draw_tree("".to_owned(), coords.0, coords.1, false);
        // mappings.append(&mut mapped);
        // }
        //
        // IMPORTANT: reverse on drawn_at is important for the bounds_for_lookup fn
        // self.drawn_at = mappings.iter().reverse()map(|&(c, n)| (n, c)).collect();
        // self.lookup = mappings.iter().map(|&e| e).collect();

        // print!("{}", termion::cursor::Goto(x, y));
        //
        // if self.selected {
        // print!("{}", termion::style::Invert);
        // }
        //
        // print!("{}", prefix);
        //
        // if prefix != "" {
        // only anchor will have blank prefix
        // if last {
        // print!("└─");
        // } else {
        // print!("├─");
        // }
        // }
        //
        // if self.stricken {
        // print!("☠");
        // } else if prefix == "" {
        // print!("⚒");
        // } else {
        // print!(" ");
        // }
        //
        // if prefix == "" {
        // print!(" ");
        // }
        //
        // print!("{}", self.content);
        //
        // if self.collapsed {
        // print!("…");
        // }
        //
        // println!("{}", termion::style::Reset);
        //
        // let mut mappings = vec![];
        // for x in x..(x + prefix.len() as u16 + self.content.len() as u16) {
        // mappings.push(((x, y), self.id));
        // }
        //
        // let mut prefix = prefix;
        // if last {
        // prefix.push_str("   ");
        // } else if prefix == "" {
        // prefix.push_str("  ");
        // } else {
        // prefix.push_str("│  ");
        // }
        // let prefix = prefix;
        //
        // let mut drawn = 1;
        // if !self.collapsed {
        // let n_children = self.children.len();
        // for (n, child) in self.children.iter().enumerate() {
        // let last = n + 1 == n_children;
        // let (child_drew, mut mapped) = child.borrow()
        // .draw_tree(prefix.clone(), x, y + drawn as u16, last);
        // mappings.append(&mut mapped);
        // drawn += child_drew;
        // }
        // }
        //
        // (drawn, mappings)
        //
    }

    pub fn height(&self, node_id: NodeID) -> Option<usize> {
        self.with_node(node_id, |node| {
            if node.collapsed {
                1
            } else {
                node.children.iter().fold(1, |acc, &c| {
                    if let Some(child_height) = self.height(c) {
                        acc + child_height
                    } else {
                        acc
                    }
                })
            }
        })
    }


    fn draw(&mut self) {
        // clear screen
        // TODO replace this with termion abstraction
        print!("\x1b[2J\x1b[H");

        let (width, bottom) = terminal_size().unwrap();

        // print header
        let header_text = self.with_node(self.drawing_root, |node| node.content.clone())
            .unwrap();

        if width > header_text.len() as u16 && bottom > 1 {
            let mut sep = format!("{}{}{}{}",
                                  cursor::Goto(0, 0),
                                  header_text,
                                  style::Invert,
                                  style::Reset);
            for _ in 0..width as usize - header_text.len() {
                sep.push('█');
            }
            println!("{}{}", cursor::Goto(0, bottom - 12), sep);
        }

        // print visible nodes
        self.draw_tree();

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
                    println!("\r{}", msg);
                }
            }
        }

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
        if self.dragging_from.is_none() {
            if let Some(&node_id) = self.lookup.get(&coords) {
                self.with_node_mut(node_id, |mut node| {
                        debug!("selected node at {:?}", coords);
                        node.selected = true;
                        node_id
                    })
                    .and_then(|id| {
                        self.last_selected = Some(node_id);
                        self.dragging_from = Some(coords);
                        Some(id)
                    })
                    .or_else(|| {
                        error!("could not find node in self.nodes from last_selected");
                        None
                    });
            }
        }
        debug!("selected no node at {:?}", coords);
        None
    }

    fn toggle_stricken(&mut self) {
        if let Some(selected_id) = self.last_selected {
            self.with_node_mut(selected_id, |node| node.toggle_stricken());
        }
    }

    fn toggle_hide_stricken(&mut self) {
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
        if let Some(selected_id) = self.last_selected.take() {
            let coords = self.drawn_at.remove(&selected_id);
            self.delete_recursive(selected_id);
            if let Some(c) = coords {
                self.click_select(c);
            }
        }
    }

    fn create_child(&mut self) {
        if let Some(selected_id) = self.last_selected {
            let node_id = self.new_node();
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
            self.with_node_mut(selected_id, |node| node.toggle_collapsed());
        }
    }

    fn create_anchor(&mut self, coords: Coords) {
        let root = self.drawing_root;
        let node_id = self.new_node();
        self.with_node_mut(node_id, |node| node.rooted_coords = coords);
        self.with_node_mut(root, |root| root.children.push(node_id));
    }

    fn backspace(&mut self) {
        if let Some(selected_id) = self.last_selected {
            self.with_node_mut(selected_id, |node| {
                let newlen = std::cmp::max(node.content.len(), 1) - 1;
                node.content = node.content.clone()[..newlen].to_owned();
            });
        }
    }

    fn append(&mut self, c: char) {
        if let Some(selected_id) = self.last_selected {
            self.with_node_mut(selected_id, |node| {
                node.content.push(c);
            });
        }
    }

    // TODO after NodeID refactor add support for moving
    // children to other trees here.
    fn move_selected(&mut self, from: Coords, to: Coords) {
        let dx = to.0 as i16 - from.0 as i16;
        let dy = to.1 as i16 - from.1 as i16;

        if let Some(selected_id) = self.last_selected {
            // find the "root" just below self.drawing_root to mod
            // the rooted_coords for.
            let mut ptr = selected_id;
            loop {
                let id = self.with_node_mut(ptr, |node| node.parent_id).unwrap();
                if id != self.drawing_root {
                    ptr = id;
                } else {
                    break;
                }
            }

            self.with_node_mut(ptr, |mut root| {
                    let coords = root.rooted_coords;
                    let nx = cmp::max(coords.0 as i16 + dx, 1) as u16;
                    let ny = cmp::max(coords.1 as i16 + dy, 1) as u16;
                    root.rooted_coords = (nx, ny);
                })
                .unwrap();
        }
    }

    fn click_select(&mut self, coords: Coords) -> Option<NodeID> {
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
        let (_, s2) = self.bounds_for_lookup(start).unwrap();
        let (_, t2) = self.bounds_for_lookup(to).unwrap();

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
            let right = (rx, left.1);
            Some((left, right))
        } else {
            None
        }
    }
}

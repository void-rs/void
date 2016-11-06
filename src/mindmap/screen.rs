use std;
use std::cmp;
use std::fs::{File, rename, remove_file};
use std::collections::BTreeMap;
use std::io::{Write, Stdout, stdout, stdin};
use std::cell::RefCell;
use std::rc::Rc;

use termion::{terminal_size, cursor, style};
use termion::color;
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};
use rand::{self, Rng};

use mindmap::{NodeID, NodeRef, Coords, Node, Meta, serialization};
use logging;

#[derive(Clone)]
struct NodeLookup {
    // anchor of selected node
    anchor: NodeRef,
    // selected node
    node: NodeRef,
}

impl PartialEq for NodeLookup {
    fn eq(&self, other: &NodeLookup) -> bool {
        self.anchor.as_ptr() == other.anchor.as_ptr() && self.node.as_ptr() == other.node.as_ptr()
    }
}

impl Eq for NodeLookup {}

pub struct Screen {
    pub work_path: Option<String>,
    stdout: Option<MouseTerminal<RawTerminal<Stdout>>>,
    dragging_from: Option<Coords>,

    // REFACTOR CHANGE
    arrows: Vec<(NodeLookup, NodeLookup)>,
    pub arrows_new: Vec<(NodeID, NodeID)>,

    last_selected: Option<NodeLookup>,
    last_selected_new: Option<NodeID>,

    drawing_arrow: Option<NodeLookup>,
    drawing_arrow_new: Option<NodeID>,

    // REFACTOR OUT
    pub anchors: BTreeMap<Coords, NodeRef>,

    // REFACTOR IN
    pub max_id: u64,
    pub drawing_root: Option<NodeID>,
    pub nodes: BTreeMap<NodeID, Node>,
    pub lookup: BTreeMap<Coords, NodeID>,
    pub drawn: BTreeMap<NodeID, Coords>,
}

impl Default for Screen {
    fn default() -> Screen {
        Screen {
            // REFACTOR OUT
            anchors: BTreeMap::new(),

            // REFACTOR CHANGE
            arrows: vec![],
            arrows_new: vec![],
            last_selected: None,
            last_selected_new: None,
            drawing_arrow: None,
            drawing_arrow_new: None,

            // REFACTOR IN
            nodes: BTreeMap::new(),
            lookup: BTreeMap::new(),
            drawn: BTreeMap::new(),
            drawing_root: None,
            stdout: None,
            dragging_from: None,
            work_path: None,
            max_id: 0,
        }
    }
}

// refactor plan
// 1. populate nodes, lookup, drawn on draw()
// 2. populate arrows_new on serialization by modifying insert arrow

impl Screen {
    fn new_node(&mut self) -> Node {
        let mut node = Node::default();
        node.id = self.max_id;
        self.max_id += 1;
        node
    }

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

    fn draw(&mut self) {
        // clear screen
        print!("\x1b[2J\x1b[H");

        let mut mappings = vec![];
        for (coords, anchor) in &self.anchors {
            let (_, mut mapped) = anchor.borrow()
                .draw_tree("".to_string(), coords.0, coords.1, false);
            mappings.append(&mut mapped);
        }

        self.drawn = mappings.iter().map(|&(c, n)| (n, c)).collect();
        self.lookup = mappings.iter().map(|&e| e).collect();

        // print logs
        let (width, bottom) = terminal_size().unwrap();
        if width > 4 && bottom > 7 {
            let mut sep = format!("{}{}logs{}",
                                  cursor::Goto(0, bottom - 6),
                                  style::Invert,
                                  style::Reset);
            for _ in 0..width - 4 {
                sep.push('█');
            }
            println!("{}{}", cursor::Goto(0, bottom - 12), sep);
            {
                let logs = logging::read_logs();
                for msg in logs.iter().rev() {
                    println!("\r{}", msg);
                }
            }
        }

        // let p = self.path((6, 16), (6, 22));
        // self.draw_path(p);

        for &(ref from, ref to) in &self.arrows {
            let path = self.path_between_nodes(from.clone(), to.clone());
            self.draw_path(path);
        }


        print!("{}", cursor::Hide);
        if let Some(mut s) = self.stdout.take() {
            s.flush().unwrap();
            self.stdout = Some(s);
        }
    }

    fn insert(&mut self, coords: Coords, node: Node) {
        let safe_coords = (cmp::max(coords.0, 1), cmp::max(coords.1, 1));
        self.anchors.insert(safe_coords, Rc::new(RefCell::new(node)));
    }

    fn coords_for_anchor(&self, node: &NodeRef) -> Option<Coords> {
        // if we switch to screen as grid of refs, use that instead
        for (&coords, anchor) in &self.anchors {
            if anchor.as_ptr() == node.as_ptr() {
                return Some(coords);
            }
        }
        None
    }

    fn coords_for_lookup(&self, lookup: NodeLookup) -> Option<Coords> {
        // if we switch to screen as grid of refs, use that instead
        // possible that a parent / anchor has been deleted
        self.coords_for_anchor(&lookup.anchor).map(|(anchor_x, anchor_y)| {
            let anchor_children = lookup.anchor.borrow().flat_visible_children();
            let mut idx = 0;
            for (i, child) in anchor_children.iter().enumerate() {
                if child.as_ptr() == lookup.node.as_ptr() {
                    idx = i + 1;
                }
            }
            (anchor_x, anchor_y + idx as u16)
        })
    }

    fn find_child_at_coords(&self, coords: Coords) -> Result<NodeLookup, String> {
        // scan possible anchors
        let mut candidate_anchors = vec![];
        for (&(x, y), anchor) in &self.anchors {
            if coords.0 >= x && coords.1 >= y && coords.1 - y < anchor.borrow().height() as u16 {
                candidate_anchors.push(((x, y), anchor.clone()));
            }
        }
        let err_string = format!("could not find node at location {:?}", coords);
        // scan possible nodes
        let mut candidate_nodes = vec![];
        for ((x, y), anchor) in candidate_anchors {
            let lookup_coords = (coords.0 - x, coords.1 - y);
            let look = if lookup_coords.1 == 0 {
                if anchor.borrow().content.len() + 1 >= lookup_coords.0 as usize {
                    Ok(anchor.clone())
                } else {
                    Err(err_string.clone())
                }
            } else {
                anchor.borrow().find_child_at_coords(0, lookup_coords)
            };
            if let Ok(node) = look {
                candidate_nodes.push(NodeLookup {
                    anchor: anchor.clone(),
                    node: node,
                });
            }
        }
        candidate_nodes.pop().ok_or(err_string)
    }

    fn pop_selected(&mut self) -> Option<NodeLookup> {
        if self.dragging_from.is_none() {
            if let Some(lookup) = self.last_selected.take() {
                lookup.node.borrow_mut().selected = false;
                Some(lookup.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn try_select(&mut self, coords: Coords) -> Option<NodeLookup> {
        if self.dragging_from.is_none() {
            if let Ok(ref lookup) = self.find_child_at_coords(coords) {
                debug!("selected node at {:?}", coords);
                lookup.node.borrow_mut().selected = true;
                self.last_selected = Some(lookup.clone());
                self.dragging_from = Some(coords);
                Some(lookup.clone())
            } else {
                debug!("selected no node at {:?}", coords);
                None
            }
        } else {
            debug!("selected no node at {:?}", coords);
            None
        }
    }

    fn toggle_stricken(&mut self) {
        if let Some(ref lookup) = self.last_selected {
            let mut node = lookup.node.borrow_mut();
            node.toggle_stricken();
        }
    }

    fn toggle_hide_stricken(&mut self) {
        if let Some(ref lookup) = self.last_selected {
            let mut node = lookup.node.borrow_mut();
            node.toggle_hide_stricken();
        }
    }

    fn delete_selected(&mut self) {
        if let Some(lookup) = self.last_selected.take() {
            let coords = self.coords_for_lookup(lookup.clone());
            let ptr = {
                lookup.anchor.as_ptr()
            };

            // clean up any arrow state
            self.arrows.retain(|&(ref from, ref to)| from != &lookup && to != &lookup);

            if ptr == lookup.node.as_ptr() {
                // nuke whole anchor
                let anchors = self.anchors
                    .clone()
                    .into_iter()
                    .filter(|&(_, ref anchor)| anchor.as_ptr() != ptr)
                    .collect();
                self.anchors = anchors;
            } else {
                lookup.anchor.borrow_mut().delete(lookup.node.clone());
            }
            if let Some(c) = coords {
                self.click_select(c);
            }
        }
    }

    fn create_child(&mut self) {
        if let Some(ref mut lookup) = self.last_selected.clone() {
            let node = self.new_node();
            let child = lookup.node.borrow_mut().attach_child(node);
            let new_lookup = NodeLookup {
                anchor: lookup.anchor.clone(),
                node: child,
            };
            self.select_node(new_lookup);
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
        if let Some(ref lookup) = self.last_selected {
            lookup.node.borrow_mut().toggle_collapsed()
        }
    }

    fn create_anchor(&mut self, coords: Coords) {
        let node = self.new_node();
        self.insert(coords, node);
    }

    fn backspace(&mut self) {
        if let Some(ref lookup) = self.last_selected {
            let mut node = lookup.node.borrow_mut();
            let newlen = std::cmp::max(node.content.len(), 1) - 1;
            node.content = node.content.clone()[..newlen].to_string();
        }
    }

    fn append(&mut self, c: char) {
        if let Some(ref lookup) = self.last_selected {
            let mut node = lookup.node.borrow_mut();
            node.content.push(c);
        }
    }

    fn move_selected(&mut self, from: Coords, to: Coords) {
        let dx = to.0 as i16 - from.0 as i16;
        let dy = to.1 as i16 - from.1 as i16;

        let anchors_clone = self.anchors.clone();
        if let Some(ref lookup) = self.last_selected {
            for (coords, value) in &anchors_clone {
                let nx = cmp::max(coords.0 as i16 + dx, 1) as u16;
                let ny = cmp::max(coords.1 as i16 + dy, 1) as u16;
                if value.as_ptr() == lookup.anchor.as_ptr() &&
                   !self.anchors.contains_key(&(nx, ny)) {
                    let anchor = self.anchors.remove(coords).unwrap();
                    self.anchors.insert((nx, ny), anchor);
                }
            }
        }
    }

    fn click_select(&mut self, coords: Coords) -> Option<NodeLookup> {
        self.pop_selected();
        let result = self.try_select((coords.0, coords.1));
        self.dragging_from.take();
        result
    }

    fn select_up(&mut self) {
        if let Some(lookup) = self.last_selected.clone() {
            if let Some(coords) = self.coords_for_lookup(lookup) {
                // to prevent selection fall-off, click old coords
                // if nothing is selected above this node
                if coords.1 > 0 {
                    self.click_select((coords.0, coords.1 - 1))
                        .or_else(|| self.click_select(coords));
                }
            }
        }
    }

    fn select_down(&mut self) {
        if let Some(lookup) = self.last_selected.clone() {
            if let Some(coords) = self.coords_for_lookup(lookup) {
                // to prevent selection fall-off, click old coords
                // if nothing is selected below this node
                self.click_select((coords.0, coords.1 + 1))
                    .or_else(|| self.click_select(coords));
            }
        }
    }

    fn select_node(&mut self, lookup: NodeLookup) {
        self.pop_selected();
        let mut node = lookup.node.borrow_mut();
        node.selected = true;
        self.last_selected = Some(lookup.clone());
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
        self.find_child_at_coords(coords).is_ok()
    }

    fn draw_arrow(&mut self) {
        if let Some(from) = self.drawing_arrow.take() {
            if let Some(arrow) = self.last_selected.clone().map(|to| (from, to)) {
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
            self.drawing_arrow = self.last_selected.clone();
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

    fn path_between_nodes(&self, start: NodeLookup, to: NodeLookup) -> Vec<Coords> {
        let (s1, s2) = self.bounds_for_lookup(start).unwrap();
        let (t1, t2) = self.bounds_for_lookup(to).unwrap();

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

    fn bounds_for_lookup(&self, lookup: NodeLookup) -> Option<(Coords, Coords)> {
        if let Some(left) = self.coords_for_lookup(lookup.clone()) {
            let mut rx = left.0;
            let node_ptr = lookup.node.as_ptr();
            while let Ok(cursor) = self.find_child_at_coords((rx, left.1)) {
                if cursor.node.as_ptr() == node_ptr {
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

    pub fn export_path_endpoints(&self) -> Vec<(Coords, Coords)> {
        let mut path_endpoints = vec![];
        for &(ref from, ref to) in &self.arrows {
            let from_coords = self.coords_for_lookup(from.clone()).unwrap();
            let to_coords = self.coords_for_lookup(to.clone()).unwrap();
            debug!("exporting arrow from: {:?} to: {:?}",
                   from_coords,
                   to_coords);
            path_endpoints.push((from_coords, to_coords));
        }
        path_endpoints
    }

    pub fn insert_path(&mut self, from: Coords, to: Coords) -> Result<(), String> {
        let from_node = try!(self.find_child_at_coords(from));
        let to_node = try!(self.find_child_at_coords(to));
        if !self.arrows.contains(&(from_node.clone(), to_node.clone())) {
            self.arrows.push((from_node, to_node));
        }
        Ok(())
    }
}

struct PrioQueue {
    to_visit: BTreeMap<u16, Vec<Coords>>,
}

impl Default for PrioQueue {
    fn default() -> PrioQueue {
        PrioQueue { to_visit: BTreeMap::new() }
    }
}

impl PrioQueue {
    fn insert(&mut self, k: u16, v: Coords) {
        let mut cur = self.to_visit.remove(&k).unwrap_or_else(|| vec![]);
        cur.push(v);
        self.to_visit.insert(k, cur);
    }
    fn pop(&mut self) -> Option<Coords> {
        if let Some((lowest_cost, _)) = self.to_visit.clone().iter().nth(0) {
            let mut cur = self.to_visit.remove(lowest_cost).unwrap_or_else(|| vec![]);
            let coords = cur.pop();
            if !cur.is_empty() {
                self.to_visit.insert(*lowest_cost, cur);
            }
            coords
        } else {
            None
        }
    }
}

fn random_color() -> String {
    use termion::color::*;
    let colors: Vec<String> = vec![format!("{}", Fg(LightGreen)),
                                   // format!("{}", Fg(LightBlack)),
                                   format!("{}", Fg(LightRed)),
                                   format!("{}", Fg(LightGreen)),
                                   format!("{}", Fg(LightYellow)),
                                   format!("{}", Fg(LightBlue)),
                                   format!("{}", Fg(LightMagenta)),
                                   format!("{}", Fg(LightCyan)),
                                   format!("{}", Fg(LightWhite))];
    let c = &*rand::thread_rng().choose(&*colors).unwrap();
    c.clone()
}


#[cfg(test)]
mod tests {
    use termion::event::{Key, Event, MouseEvent, MouseButton};

    use quickcheck::{Arbitrary, Gen, QuickCheck, StdGen};
    use rand;

    use super::*;

    #[derive(Debug, Clone)]
    struct Op {
        event: Event,
    }

    impl Arbitrary for Op {
        fn arbitrary<G: Gen>(g: &mut G) -> Op {
            let (c, x, y) = (g.gen::<char>(), g.gen::<u16>(), g.gen::<u16>());
            let events = vec![
                Event::Key(Key::Char(c)),
                Event::Key(Key::Alt('\u{1b}')),
                Event::Key(Key::Ctrl(c)),
                Event::Key(Key::Up),
                Event::Key(Key::Down),
                Event::Key(Key::Backspace),
                Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)),
                Event::Mouse(MouseEvent::Release(x, y)),
                Event::Mouse(MouseEvent::Hold(x, y)),
            ];
            Op { event: *g.choose(&*events).unwrap() }
        }
    }


    #[derive(Debug, Clone)]
    struct OpVec {
        ops: Vec<Op>,
    }

    impl Arbitrary for OpVec {
        fn arbitrary<G: Gen>(g: &mut G) -> OpVec {
            let mut ops = vec![];
            for _ in 0..g.gen_range(1, 100) {
                ops.push(Op::arbitrary(g));
            }
            OpVec { ops: ops }
        }

        fn shrink(&self) -> Box<Iterator<Item = OpVec>> {
            let mut smaller = vec![];
            for i in 0..self.ops.len() {
                let mut clone = self.clone();
                clone.ops.remove(i);
                smaller.push(clone);
            }

            Box::new(smaller.into_iter())
        }
    }

    fn prop_handle_events(ops: OpVec) -> bool {
        let mut screen = Screen::default();
        for op in &ops.ops {
            screen.handle_event(op.event);
            screen.draw();
        }
        true
    }

    #[test]
    // #[ignore]
    fn qc_merge_converges() {
        QuickCheck::new()
            .gen(StdGen::new(rand::thread_rng(), 1))
            .tests(1_000)
            .max_tests(10_000)
            .quickcheck(prop_handle_events as fn(OpVec) -> bool);
    }
}

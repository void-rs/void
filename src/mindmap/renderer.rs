use std;
use std::cmp;
use std::collections::{BTreeMap, HashMap, BinaryHeap};

use termion::{terminal_size, cursor, style, clear};
use termion::color;

use time;

use plot::plot_sparkline;
use logging;
use mindmap::{cost, Screen, NodeID, Coords, Node, random_color, Dir, ScreenDesc};

pub struct Renderer<'a>(pub &'a mut Screen);

impl<'a> Renderer<'a> {
    pub fn draw(&self) -> ScreenDesc {
        trace!("draw()");
        print!("{}", clear::All);
        // print header
        self.print_header();

        // print visible nodes
        let (lookup, drawn_at) = self.draw_children_of_root();

        // print logs
        let (width, bottom) = terminal_size().unwrap();
        if self.0.show_logs && width > 4 && bottom > 7 {
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
        for &(ref from, ref to) in &self.0.arrows {
            let (path, (direction1, direction2)) =
                self.path_between_nodes(*from, *to, &lookup, &drawn_at);
            self.draw_path(path, direction1, direction2);
        }

        print!("{}", cursor::Hide);

        (lookup, drawn_at)
    }

    fn draw_children_of_root(&self) -> ScreenDesc {
        trace!("draw_children_of_root()");
        let anchors = self.0.with_node(self.0.drawing_root, |n| n.children.clone()).unwrap();
        trace!("drawing children of root({}): {:?}",
               self.0.drawing_root,
               anchors);
        let (mut lookup, mut drawn_at) = (HashMap::new(), HashMap::new());
        for child_id in anchors {
            let coords = self.0.with_node(child_id, |n| n.rooted_coords).unwrap();
            let hide_stricken = self.0.with_node(self.0.drawing_root, |n| n.hide_stricken).unwrap();
            self.draw_node(child_id,
                           "".to_owned(),
                           coords,
                           false,
                           hide_stricken,
                           &mut lookup,
                           &mut drawn_at);
        }
        (lookup, drawn_at)
    }

    // recursively draw node and children, returning how many have been drawn
    fn draw_node(&self,
                 node_id: NodeID,
                 prefix: String,
                 coords: Coords,
                 last: bool,
                 hide_stricken: bool,
                 mut lookup: &mut HashMap<Coords, NodeID>,
                 mut drawn_at: &mut HashMap<NodeID, Coords>)
                 -> usize {
        trace!("draw_node({})", node_id);
        let (x, y) = coords;
        let node = self.0.with_node(node_id, |n| n.clone()).unwrap();
        if node.stricken && hide_stricken {
            return 0;
        }
        let (_, bottom) = terminal_size().unwrap();
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
        drawn_at.insert(node_id, (x, y));
        for x in (x..(x + 3 + prefix.len() as u16 + node.content.len() as u16)).rev() {
            trace!("inserting {:?} at {:?}", node_id, (x, y));
            lookup.insert((x, y), node_id);
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
                                                node.hide_stricken,
                                                &mut lookup,
                                                &mut drawn_at);
                drawn += child_drew;
            }
        }
        drawn
    }

    fn draw_path(&self, path: Vec<Coords>, start_dir: Dir, dest_dir: Dir) {
        trace!("draw_path({:?}, {:?}, {:?})", path, start_dir, dest_dir);
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

    fn print_header(&self) {
        trace!("print_header()");
        let mut header_text = self.0
            .with_node(self.0.drawing_root, |node| node.content.clone())
            .unwrap();

        let now = time::get_time().sec as u64;
        let day_in_sec = 60 * 60 * 24;
        let last_week = now - (day_in_sec * 7);
        let tasks_finished_in_last_week = self.0.recursive_child_filter_map(self.0.drawing_root,
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

    fn path_between_nodes(&self,
                          start: NodeID,
                          to: NodeID,
                          lookup: &HashMap<Coords, NodeID>,
                          drawn_at: &HashMap<NodeID, Coords>)
                          -> (Vec<Coords>, (Dir, Dir)) {
        trace!("getting path between {} and {}", start, to);
        let startbounds = self.bounds_for_lookup(start, lookup, drawn_at);
        let tobounds = self.bounds_for_lookup(to, lookup, drawn_at);
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
        let (width, bottom) = terminal_size().unwrap();
        if start.0 >= width || dest.0 >= width || start.1 >= bottom || dest.1 >= bottom {
            trace!("coordinate for arrow is off-self.0reen, returning no path");
            return vec![];
        }
        trace!("path({:?}, {:?} (self.0reen size: {} x {})",
               start,
               dest,
               width,
               bottom);
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
                if (!(neighbor.0 >= width) && !(neighbor.1 >= bottom) &&
                    !self.0.occupied(neighbor) || neighbor == dest) &&
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
    // value in self.0.drawn_at
    fn bounds_for_lookup(&self,
                         node_id: NodeID,
                         lookup: &HashMap<Coords, NodeID>,
                         drawn_at: &HashMap<NodeID, Coords>)
                         -> Option<(Coords, Coords)> {
        if let Some(&left) = drawn_at.get(&node_id) {
            let mut rx = left.0;
            while let Some(&cursor) = lookup.get(&(rx + 1, left.1)) {
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

use {Coords, NodeID, Meta, random_fg_color};

#[derive(Debug,Clone)]
pub struct Node {
    pub rooted_coords: Coords,
    pub parent_id: NodeID,
    pub id: NodeID,
    pub content: String,
    pub children: Vec<NodeID>,
    pub selected: bool,
    pub collapsed: bool,
    pub stricken: bool,
    pub hide_stricken: bool,
    pub meta: Meta,
    pub free_text: Option<String>,
    pub color: String,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            rooted_coords: (1, 1),
            id: 0,
            parent_id: 0,
            content: String::new(),
            children: vec![],
            selected: false,
            collapsed: false,
            stricken: false,
            hide_stricken: false,
            meta: Meta::default(),
            free_text: None,
            color: random_fg_color(),
        }
    }
}

impl Node {
    pub fn toggle_collapsed(&mut self) {
        if self.collapsed {
            self.collapsed = false;
        } else {
            self.collapsed = true;
        }
    }

    pub fn toggle_stricken(&mut self) {
        if self.stricken {
            self.stricken = false;
            self.meta.unfinish();
        } else {
            self.stricken = true;
            self.meta.finish();
        }
    }

    pub fn toggle_hide_stricken(&mut self) {
        if self.hide_stricken {
            self.hide_stricken = false;
        } else {
            self.hide_stricken = true;
        }
    }
}

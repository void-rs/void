use crate::{random_fg_color, Coords, Meta, NodeID};

#[derive(Debug, Clone)]
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
    pub auto_arrange: bool,
    pub url: Option<String>,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            rooted_coords: (1, 2),
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
            url: None,
            color: random_fg_color(),
            auto_arrange: true,
        }
    }
}

impl Node {
    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    pub fn toggle_stricken(&mut self) {
        if self.stricken {
            self.meta.unfinish();
        } else {
            self.meta.finish();
        }
        self.stricken = !self.stricken;
    }

    pub fn toggle_hide_stricken(&mut self) {
        self.hide_stricken = !self.hide_stricken;
    }

    pub fn new_from(other: &Self) -> Self {
        Node {
            color: random_fg_color(),
            id: 0,
            parent_id: 0,
            selected: false,
            ..other.clone()
        }
    }
}

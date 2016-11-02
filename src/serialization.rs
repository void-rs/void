use std;
use std::io::{Error, ErrorKind};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};

use {Screen, Node, NodeRef, Content};

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub struct SerScreen {
    pub anchors: BTreeMap<(u16, u16), SerNode>,
}

impl SerScreen {
    fn deserialize(&self) -> Screen {
        let mut screen = Screen::default();
        let mut anchors = BTreeMap::new();
        for (coords, anchor) in &self.anchors {
            anchors.insert(*coords, anchor.deserialize());
        }
        screen.anchors = anchors;
        screen
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub struct SerNode {
    pub content: Content,
    pub children: Vec<SerNode>,
    pub collapsed: bool,
    pub stricken: bool,
}

impl SerNode {
    fn deserialize(&self) -> NodeRef {
        let children = self.children
            .iter()
            .map(|ser_child| ser_child.deserialize())
            .collect();
        Rc::new(RefCell::new(Node {
            content: self.content.clone(),
            children: children,
            selected: false,
            collapsed: self.collapsed,
            stricken: self.stricken,
        }))
    }
}

pub fn serialize_screen(screen: &Screen) -> Vec<u8> {
    let serialized_screen = {
        let mut ser_anchors = BTreeMap::new();
        for (coords, anchor) in &screen.anchors {
            ser_anchors.insert(*coords, serialize_node(anchor.clone()));
        }
        SerScreen { anchors: ser_anchors }
    };
    encode(&serialized_screen, SizeLimit::Infinite).unwrap()
}

fn serialize_node(node_ref: NodeRef) -> SerNode {
    let node = node_ref.borrow();
    let ser_children = node.children
        .iter()
        .map(|child| serialize_node(child.clone()))
        .collect();
    SerNode {
        content: node.content.clone(),
        children: ser_children,
        collapsed: node.collapsed,
        stricken: node.stricken,
    }
}

pub fn deserialize_screen(data: Vec<u8>) -> std::io::Result<Screen> {
    let ser_screen: Result<SerScreen, _> = decode(&data[..])
        .map_err(|_| Error::new(ErrorKind::Other, "no path provided"));
    info!("loaded saved data");
    ser_screen.map(|ss| ss.deserialize())
}

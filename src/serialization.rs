use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use protobuf::{self, Message};

use {Screen, Node, NodeRef, Content, Meta, pb};

pub fn serialize_screen(screen: &Screen) -> Vec<u8> {
    let mut screen_pb = pb::Screen::default();
    screen_pb.set_max_id(screen.max_id);
    let anchors = screen.anchors
        .iter()
        .map(|(&(x, y), anchor)| {
            let mut anchor_pb = pb::Anchor::default();
            anchor_pb.set_x(x as u32);
            anchor_pb.set_y(y as u32);
            let head = serialize_node(anchor.clone());
            anchor_pb.set_head(head);
            anchor_pb
        })
        .collect();
    screen_pb.set_anchors(protobuf::RepeatedField::from_vec(anchors));
    let arrows = screen.export_path_endpoints()
        .iter()
        .map(|&(from, to)| {
            let mut arrow_pb = pb::Arrow::default();
            arrow_pb.set_from_x(from.0 as u32);
            arrow_pb.set_from_y(from.1 as u32);
            arrow_pb.set_to_x(to.0 as u32);
            arrow_pb.set_to_y(to.1 as u32);
            arrow_pb
        })
        .collect();
    screen_pb.set_arrows(protobuf::RepeatedField::from_vec(arrows));
    screen_pb.write_to_bytes().unwrap()
}

fn serialize_node(node_ref: NodeRef) -> pb::Node {
    let node = node_ref.borrow();
    let children_pb = node.children
        .iter()
        .map(|child| serialize_node(child.clone()))
        .collect();
    let mut node_pb = pb::Node::default();
    if let Content::Text { ref text } = node.content {
        // TODO handle other content
        node_pb.set_text(text.clone());
    }
    node_pb.set_children(protobuf::RepeatedField::from_vec(children_pb));
    node_pb.set_collapsed(node.collapsed);
    node_pb.set_stricken(node.stricken);
    node_pb.set_hide_stricken(node.hide_stricken);
    node_pb
}

pub fn deserialize_node(node_pb: pb::Node) -> NodeRef {
    let children = node_pb.get_children()
        .iter()
        .cloned()
        .map(deserialize_node)
        .collect();
    Rc::new(RefCell::new(Node {
        content: Content::Text { text: node_pb.get_text().to_string() },
        children: children,
        selected: false,
        collapsed: node_pb.get_collapsed(),
        stricken: node_pb.get_stricken(),
        hide_stricken: node_pb.get_hide_stricken(),
        meta: Meta::default(), // TODO serialize this forreal
    }))

}

pub fn deserialize_screen(data: Vec<u8>) -> Result<Screen, protobuf::ProtobufError> {
    let screen_pb: pb::Screen = try!(protobuf::parse_from_bytes(&*data));
    let anchors: BTreeMap<(u16, u16), NodeRef> = screen_pb.get_anchors()
        .iter()
        .cloned()
        .map(|mut anchor_pb| {
            let (x, y) = (anchor_pb.get_x(), anchor_pb.get_y());
            let head = anchor_pb.take_head();
            let node = deserialize_node(head);
            ((x as u16, y as u16), node)
        })
        .collect();
    let mut screen = Screen::default();
    screen.anchors = anchors;
    for arrow_pb in screen_pb.get_arrows().iter() {
        let from = (arrow_pb.get_from_x() as u16, arrow_pb.get_from_y() as u16);
        let to = (arrow_pb.get_to_x() as u16, arrow_pb.get_to_y() as u16);
        if let Err(e) = screen.insert_path(from, to) {
            error!("failed to deserialize arrows: {}", e);
        }
    }
    Ok(screen)
}

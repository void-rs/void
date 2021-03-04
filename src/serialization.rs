use protobuf::{self, Message};

use crate::{pb, random_fg_color, Meta, Node, Screen};

pub fn serialize_screen(screen: &Screen) -> Vec<u8> {
    let mut screen_pb = pb::Screen::default();
    screen_pb.set_max_id(screen.max_id);
    let nodes = screen
        .nodes
        .iter()
        .map(|(_, node)| serialize_node(node))
        .collect();
    screen_pb.set_nodes(protobuf::RepeatedField::from_vec(nodes));
    let arrows = screen
        .arrows
        .iter()
        .map(|&(from, to, _)| {
            let mut arrow_pb = pb::Arrow::default();
            arrow_pb.set_from_node(from);
            arrow_pb.set_to_node(to);
            arrow_pb
        })
        .collect();
    screen_pb.set_arrows(protobuf::RepeatedField::from_vec(arrows));
    screen_pb.write_to_bytes().unwrap()
}

fn serialize_meta(meta: &Meta) -> pb::Meta {
    let mut meta_pb = pb::Meta::default();
    meta_pb.set_gps(pb::Gps::default());
    meta_pb.set_ctime(meta.ctime);
    meta_pb.set_mtime(meta.mtime);
    if let Some(finish_time) = meta.finish_time {
        meta_pb.set_finish_time(finish_time);
    }
    let mut tags = vec![];
    for (tagk, tagv) in &meta.tags {
        let mut tag = pb::Tag::default();
        tag.set_key(tagk.clone());
        tag.set_value(tagv.clone());
        tags.push(tag);
    }
    meta_pb.set_tags(protobuf::RepeatedField::from_vec(tags));
    meta_pb
}

fn serialize_node(node: &Node) -> pb::Node {
    let mut node_pb = pb::Node::default();
    node_pb.set_id(node.id);
    node_pb.set_text(node.content.clone());
    node_pb.set_children(node.children.clone());
    node_pb.set_collapsed(node.collapsed);
    node_pb.set_stricken(node.stricken);
    node_pb.set_hide_stricken(node.hide_stricken);
    node_pb.set_parent_id(node.parent_id);
    node_pb.set_x(u32::from(node.rooted_coords.0));
    node_pb.set_y(u32::from(node.rooted_coords.1));
    node_pb.set_meta(serialize_meta(&node.meta));
    node_pb.set_auto_arrange(node.auto_arrange);
    if let Some(ref free_text) = node.free_text {
        node_pb.set_free_text(free_text.to_owned());
    }
    node_pb
}

fn deserialize_meta(meta_pb: &pb::Meta) -> Meta {
    Meta {
        ctime: meta_pb.get_ctime(),
        mtime: meta_pb.get_mtime(),
        finish_time: if meta_pb.has_finish_time() {
            Some(meta_pb.get_finish_time())
        } else {
            None
        },
        due: if meta_pb.has_due() {
            Some(meta_pb.get_due())
        } else {
            None
        },
        tags: meta_pb
            .get_tags()
            .iter()
            .map(|tag| (tag.get_key().to_owned(), tag.get_value().to_owned()))
            .collect(),
    }
}

fn deserialize_node(node_pb: &pb::Node) -> Node {
    Node {
        parent_id: node_pb.get_parent_id(),
        rooted_coords: (node_pb.get_x() as u16, node_pb.get_y() as u16),
        content: node_pb.get_text().to_owned(),
        children: node_pb.get_children().to_vec(),
        selected: node_pb.get_selected(),
        collapsed: node_pb.get_collapsed(),
        stricken: node_pb.get_stricken(),
        hide_stricken: node_pb.get_hide_stricken(),
        meta: deserialize_meta(node_pb.get_meta()),
        id: node_pb.get_id(),
        free_text: if node_pb.has_free_text() {
            Some(node_pb.get_free_text().to_owned())
        } else {
            None
        },
        color: random_fg_color(),
        auto_arrange: node_pb.get_auto_arrange(),
    }
}

pub fn deserialize_screen(data: Vec<u8>) -> Result<Screen, protobuf::ProtobufError> {
    let screen_pb: pb::Screen = protobuf::parse_from_bytes(&*data)?;
    let mut screen = Screen::default();
    screen.max_id = screen_pb.get_max_id();
    screen.nodes = screen_pb
        .get_nodes()
        .iter()
        .map(|node_pb| {
            let node = deserialize_node(node_pb);
            screen.tag_db.reindex(node.id, node.content.clone());
            (node.id, node)
        })
        .collect();

    screen.arrows = screen_pb
        .get_arrows()
        .iter()
        .map(|arrow_pb| {
            let from = arrow_pb.get_from_node();
            let to = arrow_pb.get_to_node();
            (from, to, random_fg_color())
        })
        .collect();
    Ok(screen)
}

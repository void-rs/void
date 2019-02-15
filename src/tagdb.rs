use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::{re_matches, NodeID};

pub struct TagDB {
    node_to_tags: HashMap<NodeID, HashSet<String>>,
    tag_to_nodes: HashMap<String, HashSet<NodeID>>,
}

impl Default for TagDB {
    fn default() -> Self {
        Self {
            node_to_tags: HashMap::new(),
            tag_to_nodes: HashMap::new(),
        }
    }
}

impl TagDB {
    pub fn reindex(&mut self, node: NodeID, text: &str) {
        lazy_static! {
            static ref RE_TAG_KEY: Regex = Regex::new(r"#([^\s=]+)*").unwrap();
            static ref RE_TAG_KEY_VALUE: Regex = Regex::new(r"#(\S+)*").unwrap();
        }

        self.remove(node);
        self.node_to_tags.insert(node, HashSet::new());
        let tags = re_matches::<String>(&RE_TAG_KEY_VALUE, text);

        for tag in &tags {
            if let Some(tags) = self.node_to_tags.get_mut(&node) {
                tags.insert(tag.clone());
            }

            let mut nodes = self.tag_to_nodes.remove(tag).unwrap_or_else(HashSet::new);

            nodes.insert(node);

            self.tag_to_nodes.insert(tag.clone(), nodes);
        }

        if text.contains('=') {
            let tags = re_matches::<String>(&RE_TAG_KEY, &text);

            for tag in &tags {
                if let Some(tags) = self.node_to_tags.get_mut(&node) {
                    tags.insert(tag.clone());
                }

                let mut nodes = self.tag_to_nodes.remove(tag).unwrap_or_else(HashSet::new);

                nodes.insert(node);

                self.tag_to_nodes.insert(tag.clone(), nodes);
            }
        }
    }

    pub fn remove(&mut self, node: NodeID) {
        if let Some(tags_to_clean) = self.node_to_tags.remove(&node) {
            for tag in &tags_to_clean {
                if let Some(nodes) = self.tag_to_nodes.get_mut(tag) {
                    nodes.remove(&node);
                }
            }
        }
    }

    pub fn tag_to_nodes(&self, tag: &str) -> Vec<NodeID> {
        let mut res = self
            .tag_to_nodes
            .get(&*tag)
            .map_or_else(|| vec![], |set| set.clone().into_iter().collect());
        res.sort();
        res
    }
}

#[test]
fn test_basic_func() {
    let mut tdb = TagDB::default();
    tdb.reindex(1, "hey #1 #there #yes=4");
    tdb.reindex(2, "hey #1=2 #yo #yes");
    tdb.reindex(3, "hey #1 #yes=ok");
    tdb.reindex(4, "hey #$");
    assert_eq!(tdb.tag_to_nodes("there"), vec![1]);
    assert_eq!(tdb.tag_to_nodes("1"), vec![1, 2, 3]);
    assert_eq!(tdb.tag_to_nodes("yes"), vec![1, 2, 3]);
    assert_eq!(tdb.tag_to_nodes("yes=ok"), vec![3]);
    assert_eq!(tdb.tag_to_nodes("$"), vec![4]);
}

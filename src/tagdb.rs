use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::{re_matches, NodeID};

pub struct TagDB {
    node_to_tags: HashMap<NodeID, HashSet<String>>,
    tag_to_nodes: HashMap<String, HashSet<NodeID>>,
}

impl Default for TagDB {
    fn default() -> TagDB {
        TagDB {
            node_to_tags: HashMap::new(),
            tag_to_nodes: HashMap::new(),
        }
    }
}

impl TagDB {
    pub fn reindex(&mut self, node: NodeID, text: String) {
        lazy_static! {
            static ref RE_TAG_KEY: Regex = Regex::new(r"#([^\s=]+)*").unwrap();
            static ref RE_TAG_KEY_VALUE: Regex = Regex::new(r"#(\S+)*").unwrap();
        }

        self.remove(node);
        self.node_to_tags.insert(node, HashSet::new());
        let tags = re_matches::<String>(&RE_TAG_KEY_VALUE, &*text);

        for tag in &tags {
            if let Some(tags) = self.node_to_tags.get_mut(&node) {
                tags.insert(tag.clone());
            }

            let mut nodes = self.tag_to_nodes.remove(tag).unwrap_or_else(HashSet::new);

            nodes.insert(node);

            self.tag_to_nodes.insert(tag.clone(), nodes);
        }

        if text.contains('=') {
            let tags = re_matches::<String>(&RE_TAG_KEY, &*text);

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
            .get(&tag.to_owned())
            .map(|set| set.clone().into_iter().collect())
            .unwrap_or_else(Vec::new);
        res.sort_unstable();
        res
    }
}

#[test]
fn test_basic_func() {
    let mut tdb = TagDB::default();
    tdb.reindex(1, "hey #1 #there #yes=4".to_owned());
    tdb.reindex(2, "hey #1=2 #yo #yes".to_owned());
    tdb.reindex(3, "hey #1 #yes=ok".to_owned());
    tdb.reindex(4, "hey #$".to_owned());
    assert_eq!(tdb.tag_to_nodes("there"), vec![1]);
    assert_eq!(tdb.tag_to_nodes("1"), vec![1, 2, 3]);
    assert_eq!(tdb.tag_to_nodes("yes"), vec![1, 2, 3]);
    assert_eq!(tdb.tag_to_nodes("yes=ok"), vec![3]);
    assert_eq!(tdb.tag_to_nodes("$"), vec![4]);
}

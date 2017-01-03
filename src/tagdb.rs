use std::collections::{HashMap, HashSet};

use regex::Regex;

use {NodeID, re_matches};

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
            static ref RE_TAG: Regex = Regex::new(r"#(\w+)*").unwrap();
        }

        self.remove(node);
        self.node_to_tags.insert(node, HashSet::new());
        let tags = re_matches::<String>(&RE_TAG, &*text);

        for tag in &tags {
            if let Some(mut tags) = self.node_to_tags.get_mut(&node) {
                tags.insert(tag.clone());
            }

            let mut nodes = self.tag_to_nodes
                .remove(tag)
                .unwrap_or_else(|| HashSet::new());

            nodes.insert(node);

            self.tag_to_nodes.insert(tag.clone(), nodes);
        }
    }

    pub fn remove(&mut self, node: NodeID) {
        if let Some(tags_to_clean) = self.node_to_tags.remove(&node) {
            for tag in &tags_to_clean {
                if let Some(mut nodes) = self.tag_to_nodes.get_mut(tag) {
                    nodes.remove(&node);
                }
            }
        }
    }

    pub fn tag_to_nodes(&self, tag: &str) -> Vec<NodeID> {
        let mut res = self.tag_to_nodes
            .get(&tag.to_owned())
            .map(|set| set.clone().into_iter().collect())
            .unwrap_or_else(|| vec![]);
        res.sort();
        res
    }
}

#[test]
fn test_basic_func() {
    let mut tdb = TagDB::default();
    tdb.reindex(1, "hey #1 #there #yes=4".to_owned());
    tdb.reindex(2, "hey #1=2 #yo #yes".to_owned());
    tdb.reindex(3, "hey #1 #yes=ok".to_owned());
    assert_eq!(tdb.tag_to_nodes("there"), vec![1]);
    assert_eq!(tdb.tag_to_nodes("1"), vec![1, 2, 3]);
    assert_eq!(tdb.tag_to_nodes("yes"), vec![1, 2, 3]);
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct Meta {
    pub id: u64,
    pub ctime: u64,
    pub mtime: u64,
    pub gps: (f64, f64),
    pub tags: HashMap<String, String>,
}

impl Default for Meta {
    fn default() -> Meta {
        Meta {
            id: 0,
            ctime: 0,
            mtime: 0,
            gps: (0.0, 0.0),
            tags: HashMap::new(),
        }
    }
}

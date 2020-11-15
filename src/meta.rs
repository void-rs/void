use std::collections::HashMap;

use crate::now;

#[derive(Debug, Clone)]
pub struct Meta {
    pub ctime: u64,
    pub mtime: u64,
    pub finish_time: Option<u64>,
    pub due: Option<u64>,
    pub tags: HashMap<String, String>,
}

impl Default for Meta {
    fn default() -> Meta {
        let now = now().as_secs();
        Meta {
            ctime: now,
            mtime: now,
            finish_time: None,
            due: None,
            tags: HashMap::new(),
        }
    }
}

impl Meta {
    pub fn bump_mtime(&mut self) {
        self.mtime = now().as_secs();
    }

    pub fn finish(&mut self) {
        self.finish_time = Some(now().as_secs());
    }

    pub fn unfinish(&mut self) {
        self.finish_time = None;
    }

    pub fn at(&self) -> u64 {
        self.finish_time.unwrap_or(self.mtime)
    }
}

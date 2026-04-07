use sha2::{Sha256, Digest};

pub struct Entry {
    pub hash: String,
    pub event: String,
}

pub struct Journal {
    pub chain: Vec<Entry>,
}

impl Journal {
    pub fn new() -> Self {
        Self { chain: Vec::new() }
    }

    pub fn append(&mut self, event: String) {
        let prev = if let Some(last) = self.chain.last() {
            last.hash.clone()
        } else {
            String::from("0")
        };

        let mut hasher = Sha256::new();
        hasher.update(prev);
        hasher.update(&event);

        let result = hasher.finalize();
        let hash = format!("{:x}", result);

        let entry = Entry { hash, event };
        println!("{} {}", entry.hash, entry.event);

        self.chain.push(entry);
    }
}

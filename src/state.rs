use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    pub data: HashMap<String, f64>,
}

impl State {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn apply(&mut self, update: HashMap<String, f64>) {
        for (k, v) in update {
            self.data.insert(k, v);
        }
    }
}

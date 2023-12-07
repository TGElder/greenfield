use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Service {
    next_id: usize,
}

impl Service {
    pub fn new() -> Service {
        Service { next_id: 0 }
    }

    pub fn next_id(&mut self) -> usize {
        let out = self.next_id;
        self.next_id = out + 1;
        out
    }
}

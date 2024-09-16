use std::time::Instant;

#[derive(Clone)]
pub struct Message {
    pub timestamp: Instant,
    pub text: String,
}

impl Message {
    pub fn new(text: String) -> Message {
        Message {
            timestamp: Instant::now(),
            text,
        }
    }
}

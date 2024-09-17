use std::time::Instant;

#[derive(Clone)]
pub struct Message {
    pub timestamp: Instant,
    pub text: String,
}

impl Message {
    pub fn new<T>(text: T) -> Message
    where
        T: Into<String>,
    {
        Message {
            timestamp: Instant::now(),
            text: text.into(),
        }
    }
}

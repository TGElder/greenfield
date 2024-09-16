use std::time::{Duration, Instant};

use tokio::sync::broadcast::Receiver;

use crate::model::message::Message;

pub struct System {
    rx: Receiver<Message>,
    messages: Vec<Message>,
    parameters: Parameters,
}

pub struct Parameters {
    pub max_duration: Duration,
    pub max_length: usize,
}

impl System {
    pub fn new(rx: Receiver<Message>, parameters: Parameters) -> System {
        System {
            rx,
            messages: vec![],
            parameters,
        }
    }

    pub fn run(&mut self) {
        self.pull();
        self.apply_max_duration();
        self.sort();
        self.apply_max_length();
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    fn pull(&mut self) {
        while let Ok(message) = self.rx.try_recv() {
            self.messages.push(message);
        }
    }

    fn apply_max_duration(&mut self) {
        let min_timestamp = Instant::now() - self.parameters.max_duration;
        self.messages
            .retain(|Message { timestamp, .. }| *timestamp >= min_timestamp);
    }

    fn sort(&mut self) {
        self.messages
            .sort_by_key(|Message { timestamp, .. }| *timestamp);
    }

    fn apply_max_length(&mut self) {
        let excess = self
            .messages
            .len()
            .saturating_sub(self.parameters.max_length);
        self.messages.drain(0..excess);
    }
}

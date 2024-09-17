use tokio::sync::broadcast::Sender;

use crate::model::message::Message;

pub struct System {
    tx: Sender<Message>,
}

impl System {
    pub fn new(tx: Sender<Message>) -> System {
        System { tx }
    }

    pub fn send<T>(&mut self, text: T)
    where
        T: Into<String>,
    {
        if let Err(e) = self.tx.send(Message::new(text)) {
            println!("Couldn't send message: {}", e);
        }
    }
}

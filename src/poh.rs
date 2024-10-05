use crate::core::Transaction;
use anyhow::Result;
use blake3::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub enum Payload {
    RegisterEvent { transaction: Transaction },
    Shutdown,
}

pub struct Event {
    pub count: u64,
    pub transaction: Transaction,
}

pub struct Poh {
    last_hash: Hash,
    record: Vec<Hash>,
    events: Vec<Event>,
    count: u64,
    receiver: Receiver<Payload>,
    sender: Sender<Payload>,
}

impl Poh {
    const DEFAULT_MSG_BUFFER_SIZE: usize = 100;

    pub fn new(msg_buff_size: Option<usize>) -> Self {
        let buffer_size = match msg_buff_size {
            Some(s) => s,
            None => Self::DEFAULT_MSG_BUFFER_SIZE,
        };

        let (tx, rx) = channel(buffer_size);

        let count = u64::default();
        let mut hasher = Hasher::new();
        hasher.update(&count.to_le_bytes());
        let hash = hasher.finalize();

        Self {
            last_hash: hash,
            record: vec![hash],
            events: Vec::new(),
            count,
            receiver: rx,
            sender: tx,
        }
    }

    pub fn get_sender(&self) -> Sender<Payload> {
        self.sender.clone()
    }

    async fn tick(&mut self) {
        let new_count = self.count + 1;
        let mut hasher = Hasher::new();
        hasher.update(self.last_hash.as_bytes());
        hasher.update(&self.count.to_le_bytes());
        let new_hash = hasher.finalize();

        self.record.push(new_hash);
        self.count = new_count;
        self.last_hash = new_hash;
    }

    fn register_event(&mut self, event: Event) {
        let mut hasher = Hasher::new();
        hasher.update(self.last_hash.as_bytes());
        hasher.update(&event.count.to_le_bytes());
        hasher.update(event.transaction.message.as_bytes());
        let new_hash = hasher.finalize();

        self.record.push(new_hash);
        self.count = event.count;
        self.last_hash = new_hash;
    }

    pub async fn run(&'static mut self) -> Result<()> {
        loop {
            tokio::select! {
                payload = self.receiver.recv() => {
                    if payload.is_none() { continue }

                    match payload.unwrap() {
                        Payload::Shutdown => break,
                        Payload::RegisterEvent {transaction} => {
                            let event = Event {count: self.count, transaction};
                            self.register_event(event);
                        }
                    }
                }

                _ = self.tick() => {}
            }
        }

        Ok(())
    }
}

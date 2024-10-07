use crate::core::Transaction;
use anyhow::Result;
use blake3::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub enum Payload {
    RegisterEvent { transaction: Transaction },
    Shutdown,
    Tick,
}

pub struct Event {
    pub transaction: Transaction,
}

pub struct Record {
    hash: Hash,
    count: u64,
    event: Option<Event>,
}

pub struct Poh {
    last_hash: Hash,
    last_count: u64,
    record: Vec<Record>,
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

        let count: u64 = 1;
        let mut hasher = Hasher::new();
        hasher.update(&count.to_le_bytes());
        let hash = hasher.finalize();
        let record = Record {
            hash,
            count,
            event: None,
        };

        Self {
            last_hash: hash,
            last_count: count,
            record: vec![record],
            receiver: rx,
            sender: tx,
        }
    }

    pub fn subscribe(&self) -> Sender<Payload> {
        self.sender.clone()
    }

    fn tick(&mut self) {
        let new_count = self.last_count + 1;

        let mut hasher = Hasher::new();
        hasher.update(self.last_hash.as_bytes());
        hasher.update(&new_count.to_le_bytes());
        let new_hash = hasher.finalize();

        self.record.push(Record {
            hash: new_hash,
            count: new_count,
            event: None,
        });
        self.last_count = new_count;
        self.last_hash = new_hash;
    }

    fn register_event(&mut self, event: Event) {
        let new_count = self.last_count + 1;

        let mut hasher = Hasher::new();
        hasher.update(self.last_hash.as_bytes());
        hasher.update(&new_count.to_le_bytes());
        hasher.update(event.transaction.message.as_bytes());
        let new_hash = hasher.finalize();

        self.record.push(Record {
            hash: new_hash,
            count: new_count,
            event: Some(event),
        });
        self.last_count = new_count;
        self.last_hash = new_hash;
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                payload = self.receiver.recv() => {
                    match payload.unwrap() {
                        Payload::Tick => {},
                        Payload::Shutdown => {
                            break;
                        },
                        Payload::RegisterEvent { transaction } => {},
                    }
                },
            }
        }

        Ok(())
    }

    pub async fn vdf(sender: Sender<Payload>) {
        loop {
            // TODO: Eliminate unwrap
            sender.send(Payload::Tick).await.unwrap();
        }
    }

    pub async fn add_transaction(sender: Sender<Payload>, transaction: Transaction) {
        // Eliminate unwrap
        sender
            .send(Payload::RegisterEvent { transaction })
            .await
            .unwrap();
    }

    pub async fn shutdown(sender: Sender<Payload>) {
        // Eliminate unwrap
        sender.send(Payload::Shutdown).await.unwrap();
    }
}

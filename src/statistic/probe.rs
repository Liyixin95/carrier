use crate::statistic::record::Record;
use std::sync::mpsc::Sender;

pub struct Probe {
    tx: Sender<Record>,
}

impl Probe {
    pub fn new(tx: Sender<Record>) -> Self {
        Self { tx }
    }

    pub fn push(&mut self, data: u32) {
        self.tx.send(Record::success(data));
    }
}

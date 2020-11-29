use std::sync::mpsc::Sender;
use crate::statistic::record::Record;

pub struct Probe {
    tx: Sender<Record>
}

impl Probe {
    pub fn new(tx: Sender<Record>) -> Self {
        Self {
            tx
        }
    }

    pub fn push(&mut self, data: u32) {
        self.tx.send(Record{});
    }

}
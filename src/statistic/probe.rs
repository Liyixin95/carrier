use std::sync::mpsc::Sender;
use crate::statistic::record::Record;

pub struct Probe {
    tx: Sender<Record>
}

impl Probe {
    pub fn new(tx: Sender<Record>) -> Self {
        Self {
            tx
        }git
    }

    pub fn push(&mut self, data: u32) {
        self.active.push(data);
    }

    pub fn read_to_collect(&mut self) -> bool {
        self.standby.len() > 0
    }
}
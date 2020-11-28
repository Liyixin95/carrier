use crate::statistic::probe::Probe;
use std::sync::mpsc::{Receiver, Sender};
use crate::statistic::record::Record;
use std::sync::mpsc;

pub struct Nexus<T> where T: FnOnce(Record) {
    rx: Receiver<Record>,
    tx: Sender<Record>,
    handle: T,
}

impl<T> Nexus<T> {
    pub fn new(handle: T) -> Self {
        let (tx,rx) = mpsc::channel();
        Self {rx,tx, handle}
    }
    pub fn spawn(&self) -> Probe {
        Probe::new(self.tx.clone())
    }

    pub fn set_handle() {}

    fn collect_loop(&self) {
        while let Ok(record) = self.rx.recv() {

        }
    }


}
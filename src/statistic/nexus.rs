use crate::statistic::probe::Probe;
use crate::statistic::record::Record;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::time::Duration;

pub struct Nexus<T>
where
    T: FnOnce(Record),
{
    rx: Receiver<Record>,
    tx: Sender<Record>,
    buffer: Vec<Record>,
    cur_time: u64,
    handle: T,
}

impl<T> Nexus<T>
where
    T: FnOnce(Record),
{
    pub fn new(handle: T) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            rx,
            tx,
            buffer: Vec::with_capacity(1024),
            cur_time: 0,
            handle,
        }
    }
    pub fn spawn(&self) -> Probe {
        Probe::new(self.tx.clone())
    }

    pub fn set_handle() {}

    fn collect_loop(&self) {
        loop {
            while let Some(record) = self.rx.try_recv() {

            }

            std::thread::sleep(Duration::from_millis(500))
        }
    }
}

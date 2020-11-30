use crate::statistic::probe::Probe;
use crate::statistic::record::Record;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crate::statistic::bundle::Bundle;

pub struct Nexus<T>
where
    T: FnOnce(Record),
{
    rx: Receiver<Record>,
    tx: Sender<Record>,
    bundle: Bundle,
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
            bundle: Bundle::new(1024),
            cur_time: 0,
            handle,
        }
    }
    pub fn spawn(&self) -> Probe {
        Probe::new(self.tx.clone())
    }

    fn collect_loop(&mut self) {
        loop {
            while let Some(record) = self.rx.try_recv() {
                self.bundle.push(record);
            }



            self.bundle.reset();

            std::thread::sleep(Duration::from_millis(500))
        }
    }
}

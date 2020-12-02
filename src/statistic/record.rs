use std::ops::Add;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Record {
    success: bool,
    latency: u32,
}

impl Record {
    pub fn success(latency: u32) -> Self {
        Self {
            success: true,
            latency
        }
    }

    pub fn failure(latency: u32) -> Self {
        Self {
            success: false,
            latency
        }
    }

    pub fn copy(&mut self, record: Record) {
        self.success = record.success;
        self.latency = record.latency;
    }
}

impl Add for Record {
    type Output = u32;

    fn add(self, rhs: Self) -> Self::Output {
        self.latency  + rhs.latency
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.latency == other.latency
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.latency.cmp(&other.latency))
    }
}
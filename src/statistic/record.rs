use std::ops::Add;
use std::cmp::Ordering;

pub struct Record {
    success: bool,
    latency: u64,
}

impl Record {
    pub fn success(latency: u64) -> Self {
        Self {
            success: true,
            latency
        }
    }

    pub fn failure(latency: u64) -> Self {
        Self {
            success: false,
            latency
        }
    }
}

impl Add for Record {
    type Output = u64;

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
use crate::statistic::record::Record;

pub struct Bundle {
    container: Vec<Record>,
    index: usize,
}

impl Bundle {
    pub fn new(cap: usize) -> Self {
        Self {
            container: Vec::with_capacity(cap),
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn push(&mut self, record: Record) {
        let idx = self.index;
        if idx >= self.container.len() {
            self.container.push(record);
        } else {
            self.container[idx].copy(record);
        }

        self.index = idx + 1;
    }

    pub fn sort(&mut self, until: u8) {
        if until > self.index as u8 {
        }
    }

    pub fn count(&self) -> u64 {
        self.index as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut bundle = Bundle::new(8);
        bundle.push(Record::success(10));
        bundle.push(Record::success(11));
        bundle.push(Record::success(12));

        assert_eq!(bundle.container[0], Record::success(10));
        assert_eq!(bundle.container[1], Record::success(11));
        assert_eq!(bundle.container[2], Record::success(12));

        bundle.reset();
        bundle.push(Record::failure(20));
        assert_eq!(bundle.container[0], Record::failure(20));
    }
}

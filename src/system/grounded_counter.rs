#![allow(dead_code)]
pub struct GroundedCounter {
    count_left: u32,
    max_count: u32,
}

impl GroundedCounter {
    pub fn new(max_count: u32) -> Self {
        Self {
            count_left: max_count,
            max_count,
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.count_left = self.max_count;
        self
    }

    // When returning boolean is true, it shows that count is expire.
    pub fn count_down(&mut self) -> bool {
        self.count_left -= 1;
        if self.count_left == 0 {
            self.reset();
            true
        } else {
            false
        }
    }

    pub fn get_count_left(&self) -> u32 {
        self.count_left
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn constructor() {
        let counter = GroundedCounter::new(100);
        assert_eq!(counter.get_count_left(), 100);
    }

    #[test]
    fn test_count_down() {
        let mut counter = GroundedCounter::new(2);
        assert_eq!(counter.count_down(), false);
        assert_eq!(counter.get_count_left(), 1);

        assert_eq!(counter.count_down(), true);
        assert_eq!(counter.get_count_left(), 2);
    }

    #[test]
    fn test_reset() {
        let mut counter = GroundedCounter::new(3);
        counter.count_down();
        counter.reset();
        assert_eq!(counter.get_count_left(), 3);
    }
}

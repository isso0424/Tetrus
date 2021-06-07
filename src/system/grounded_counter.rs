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
}

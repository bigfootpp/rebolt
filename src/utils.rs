pub struct IdGenerator {
    next_id: usize,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn next(&mut self) -> usize {
        let current = self.next_id;
        self.next_id += 1;
        current
    }
}

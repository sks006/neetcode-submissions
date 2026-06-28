pub struct MyHashSet {
    bitmap: Vec<u64>,
}

impl MyHashSet {
    pub fn new() -> Self {
        // 1_000_000 / 64 = 15_625, so we need 15_626 slots for index 1_000_000.
        Self {
            bitmap: vec![0; 15_626],
        }
    }

    pub fn add(&mut self, key: i32) {
        let key = key as usize;          // handle only non‑negative keys
        let block = key / 64;
        let offset = key % 64;
        self.bitmap[block] |= 1 << offset;   // offset is usize, shift works
    }

    pub fn remove(&mut self, key: i32) {
        let key = key as usize;
        let block = key / 64;
        let offset = key % 64;
        self.bitmap[block] &= !(1 << offset);
    }

    pub fn contains(&self, key: i32) -> bool {
        let key = key as usize;
        let block = key / 64;
        let offset = key % 64;
        (self.bitmap[block] & (1 << offset)) != 0
    }
}
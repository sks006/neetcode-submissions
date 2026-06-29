struct MyHashMap {
    table:Vec<i32>
}

impl MyHashMap {
    pub fn new() -> Self {
        // [INITIALIZATION BOUNDARY]
        // 1. The maximum key is 1,000,000. 
        const Maximum:usize=1000000;
        // 2. Allocate a Vec of size 1,000,001 to inclusively cover the key space.
        let table = vec![-1; Maximum + 1];
        // 3. Initialize all indices with the integer `-1` to represent an empty mapping.
        Self { table }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // [STATE MUTATION BOUNDARY]
        // 1. Cast `key` to the architecture-specific memory size (`usize`).
            let index=key as unsize;
        // 2. Directly access the vector at the casted index.

        // 3. Overwrite the memory location with the new `value`.
             self.tanle[index]=value
    }

    pub fn get(&self, key: i32) -> i32 {
        // [READ-ONLY VERIFICATION BOUNDARY]
        // 1. Cast `key` to `usize`.
        let index= key as unsize;
        // 2. Return a copy of the integer stored at `table[index]`. 
       self.table[index]
        // Note: If the key was never inserted, this naturally returns the default `-1`.
    }

    pub fn remove(&mut self, key: i32) {
        // [DELETION BOUNDARY]
        // 1. Cast `key` to `usize`.
        let index= key as unsize;
        // 2. Overwrite the memory location with `-1` to logically drop the mapping.
        self.table[index]=-1
    }
}
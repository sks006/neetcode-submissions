use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        
        // 1. Count frequencies
        let mut freq = HashMap::new();
        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        // 2. Min-heap to keep the top k frequent elements
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for (num, count) in freq {
            heap.push(Reverse((count, num)));
            if heap.len() > k {
                heap.pop();
            }
        }

        // 3. Extract the numbers
        heap.into_iter()
            .map(|Reverse((_, num))| num)
            .collect()
    }
}
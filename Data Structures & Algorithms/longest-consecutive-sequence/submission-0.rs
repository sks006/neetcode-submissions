impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

    // 1. Insert all numbers into a HashSet to remove duplicates and allow O(1) lookups
    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut longest_streak = 0;
        
    for &num in &num_set{
            if !num_set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;

            // 4. Count how far the consecutive sequence goes
            while num_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }

            // 5. Keep track of the maximum length found so far
           longest_streak = max(longest_streak, current_streak);

            }
             
        }
    longest_streak
    }
}
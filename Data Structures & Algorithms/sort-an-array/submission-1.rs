impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
   let mut temp = 0;
    for x in 0..nums.len() {
        for y in 1..nums.len() {
            if nums[x] > nums[y] {
                temp = nums[y];
                nums[y] = nums[x];
                nums[x] = temp;
            }
        }
    
    }
    }
    nums
}

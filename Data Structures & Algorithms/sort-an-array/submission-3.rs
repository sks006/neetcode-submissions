impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
   let mut temp = 0;
     for x in 0..nums.len() {
        for y in x+1..nums.len() {   // full range from 1 to len-1
            if nums[x] > nums[y] {
                temp = nums[y];
                nums[y] = nums[x];
                nums[x] = temp;
            }
        }
    }
    nums
    }
    
}

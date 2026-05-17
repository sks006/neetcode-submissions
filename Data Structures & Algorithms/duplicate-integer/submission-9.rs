impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let n= nums.len();

        for i in 0..n{
         for j in  i+1..n {
            if nums[i]==nums[j]{
                return true
            }
         }
        }false
        
    }
}

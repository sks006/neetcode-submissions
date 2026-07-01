impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {

        for i in 0..nums.len(){
            for j in i + 1..nums.len(){
              
                if nums[i] > nums[j] {
                // standard swap using a temporary variable
                let temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
            }
               
                
            } 
        }

    }
}

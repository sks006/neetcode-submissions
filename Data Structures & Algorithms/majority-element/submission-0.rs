impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
    let count_ref: &mut i32 = counts.entry(num).or_insert(0);
*count_ref += 1;
         counts
        .into_iter()
        .max_by_key(|&(_num, count)| count)
        .map(|(num, _)| num)
        .unwrap()
    }
}

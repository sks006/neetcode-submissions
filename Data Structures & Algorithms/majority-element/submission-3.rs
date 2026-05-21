impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
let mut counts: HashMap<i32, i32> = HashMap::new();
  /*Counting
  counts.entry(num).or_insert(0) gets the current count or inserts 0.
*... += 1 increments that count.*/
  
  for num in nums {
    let count_ref: &mut i32 = counts.entry(num).or_insert(0);
    
    *count_ref += 1;
}

/*Finding the majority

into_iter() takes ownership of the map and gives key-value pairs.
max_by_key(...) finds the pair with the largest count (the majority element, because it appears more than n/2 times, so its count must be the largest).
map(|(num, _)| num) extracts just the number.
unwrap() is safe because the input is never empty (per problem constraints).*/
counts
    .into_iter()
    .max_by_key(|&(_num, count)| count)
    .map(|(num, _)| num)
    .unwrap()
    }
}

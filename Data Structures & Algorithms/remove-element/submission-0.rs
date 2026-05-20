impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    //sort the vector
        let mut map=HashMap::new();
        for s in nums{
         let key= s.values().collect(),
         key.sort(),
         map.values(key).insert(Vec::new()).push()   
        }
    //find the biggest number
    let max=map.iter().max();
        for x in map{
           
            max.retain(|&x| x != to_remove);
           
        }
    }
}

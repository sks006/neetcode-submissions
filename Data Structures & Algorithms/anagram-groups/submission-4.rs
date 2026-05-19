impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs{
            let mut key:Vec<char>=s.chars().collect();
            key.sort();
            map.entry(key).or_insert(Vec::new()).push(s);
        }

        map.into_value().collect();
    
    }
}

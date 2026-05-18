impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty(){return String::new();};

    let first = &strs[0];                    // reference string
    let bytes = first.as_bytes();            // its bytes
    let mut prefix_len = 0;

    for (idx, &b) in bytes.iter().enumerate() {
        for other in &strs[1..] {            // check all other strings
            if other.as_bytes().get(idx) != Some(&b) {
                return first[..prefix_len].to_string();
            }
        }
        prefix_len += 1;                     // all strings matched at this position
    }

    first.to_string()
}
    }


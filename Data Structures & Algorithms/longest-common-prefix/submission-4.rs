impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty(){return String::new();};

    for (idx, b) in first.as_bytes().iter().enumerate() {
    let b = *b; // dereference to u8
    for other in &strs[1..] {
        // check if other is long enough
        if idx >= other.as_bytes().len() {
            return first[..idx].to_string();
        }
        // check if byte is different
        if other.as_bytes()[idx] != b {
            return first[..idx].to_string();
        }
    }
}
    }
}

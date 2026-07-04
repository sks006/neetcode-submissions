impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
       
        let mut encoded = String::new();
    for s in &strs {
        encoded.push_str(&format!("{}#{}", s.len(), s));
    }
    encoded
    }

    pub fn decode(s: String) -> Vec<String> {

    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // 1. Find the '#' delimiter that separates length from string
        let mut j = i;
        while j < bytes.len() && bytes[j] != b'#' {
            j += 1;
        }

        // 2. Parse the length (digits before '#')
        let length: usize = std::str::from_utf8(&bytes[i..j])
            .unwrap()
            .parse()
            .unwrap();

        // 3. Skip the '#'
        j += 1;

        // 4. Extract exactly `length` bytes as the original string
        let end = j + length;
        let word = &s[j..end];
        result.push(word.to_string());

        // 5. Move to the next encoded chunk
        i = end;
    }

    result
    }
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let asciCode1= s.as_bytes();
        let mut x = Vec::new();

        for byte in asciCode1{
            x.push(*byte);
        }
        x.sort();   // sorts in ascending order (smallest to largest)
        
        let asciCode2= t.as_bytes();
        let mut y = Vec::new();
        for byte in asciCode2{
            y.push(*byte);
        }
        y.sort(); 

        if x==y{
            return true
        }false
    }
}

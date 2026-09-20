impl Solution {
    fn is_palindromic(s: &str) -> bool {
        // Edge case
        if s.len() <= 1 { return true; }
        // Normal case
        let half = s.len()/2;
        for (idx, byte) in s.as_bytes().iter().take(half).enumerate() {
            let byte_rev = s.as_bytes()[s.len() - 1 - idx];
            if *byte != byte_rev { return false; }
        }
        true
    }
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 { return s; }
        let mut max_str: &str = "";
        for idx in 0..s.len() {
            for idx_rev in (idx..s.len()).rev() {
                let string = &s[idx..idx_rev + 1];
                if string.len() <= max_str.len() { continue; }
                if !Self::is_palindromic(string) { continue; }
                max_str = string;
            }            
        }
        // let s = String::from("Hello");
        max_str.to_string()
    }
}
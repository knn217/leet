impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        for (left, ch) in haystack.chars().enumerate() {
            let right = left + needle.len();
            if right > haystack.len() { break; }
            if haystack[left..right] == needle[..] {
                return left as i32;
            }
        }
        -1
    }
}
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut window_hash = HashMap::new();
        let mut start = 0;
        let mut res = 0;
    
        for (end, ch) in s.chars().enumerate() {
            if let Some(pos) = window_hash.get(&ch) {
                start = start.max(*pos);
            }
            res = res.max(end + 1 - start);
            window_hash.insert(ch, end + 1);
        }
        res as i32
    }
}
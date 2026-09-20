use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut sum = 0;
        let mut curr_val = 0;
        for (idx, ch) in s.chars().rev().enumerate() {
            let prev_val = curr_val;
            curr_val = *map.get(&ch).unwrap();
            if curr_val >= prev_val {
                sum += curr_val;
            } else {
                sum -= curr_val;
            }
        }
        sum
    }
}
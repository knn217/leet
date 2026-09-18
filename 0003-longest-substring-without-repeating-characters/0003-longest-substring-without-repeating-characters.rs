use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut map = vec![usize::MAX; 128]; // Vec of 128 element, init as 0
        let mut start: usize = 0;
        for (i, c) in s.chars().enumerate() {
            let b = c as usize;
            if b > map.len() {
                map.resize(b, usize::MAX);
            }
            if usize::MAX != map[b] {
                if map[b] >= start {
                    // println!("repeated idx {}, updating starting point from {} to {}", map[b], start, map[b] + 1);
                    start = map[b] + 1;
                }
                // println!("deprecated idx {}, updating to {}", map[b], i);
                map[b] = i;
            } else {
                // println!("Unintialized idx, init to {}", i);
                map[b] = i;
            }
            // println!("{}", c);
            // println!("{}, {}, {}", max, i, start);
            max = cmp::max(max, (i + 1 - start));
        }
        return max as i32;
    }
}
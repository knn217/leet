impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = vec![usize::MAX; 128]; // Vec of 128 element, use MAX as init value
        let mut start = 0;
        let mut max = 0;
        for (end, ch) in s.chars().enumerate() {
            let b = ch as usize;
            if usize::MAX != map[b] {
                // Found valid index for current char -> a repeat
                start = start.max(map[b]); // start takes the max of the 2 repeats
            }
            max = max.max(end + 1 - start);
            // Update the index in map
            map[b] = end + 1;
        }
        max as i32
    }
}
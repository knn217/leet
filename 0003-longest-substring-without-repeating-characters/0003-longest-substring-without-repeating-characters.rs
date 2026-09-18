
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = vec![usize::MAX; 128]; // Vec of 128 element, init as 0
        let mut start = 0;
        let mut max = 0;
        for (end, ch) in s.chars().enumerate() {
            let b = ch as usize;
            if usize::MAX != map[b] {
                start = start.max(map[b]);
            }
            max = max.max(end + 1 - start);
            map[b] = end + 1;
        }
        max as i32
    }
}
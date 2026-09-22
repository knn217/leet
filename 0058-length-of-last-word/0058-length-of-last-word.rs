impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut len = 0;
        for ch in s.chars().rev() {
            if ch != ' ' { len += 1; }
            else if len != 0 { break; }
        }
        len
    }
}
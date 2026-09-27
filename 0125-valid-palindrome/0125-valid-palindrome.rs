impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        const MIN: u8 = b'a';
        const MAX: u8 = b'z';
        const MIN_CAP: u8 = b'A';
        const MAX_CAP: u8 = b'Z';
        const MIN_NUM: u8 = b'0';
        const MAX_NUM: u8 = b'9';
        let mut idx_left = 0;
        let mut idx_right = s.len() - 1;
        while idx_left < idx_right {
            let mut char_left = s.as_bytes()[idx_left];
            let mut char_right = s.as_bytes()[idx_right];
            // println!("char_left: {}, char_right: {}", char_left as char, char_right as char);
            if (MIN_CAP..=MAX_CAP).contains(&char_left) { char_left += MIN; char_left -= MIN_CAP; }
            if (MIN_CAP..=MAX_CAP).contains(&char_right) { char_right += MIN; char_right -= MIN_CAP; }
            // println!("char_left: {}, char_right: {}", char_left as char, char_right as char);
            if !(MIN..=MAX).contains(&char_left) && !(MIN_NUM..=MAX_NUM).contains(&char_left) { idx_left += 1; continue; }
            if !(MIN..=MAX).contains(&char_right) && !(MIN_NUM..=MAX_NUM).contains(&char_right) { idx_right -= 1; continue; }
            if char_left != char_right { return false; }
            idx_left += 1;
            idx_right -= 1;
        }
        true
    }
}
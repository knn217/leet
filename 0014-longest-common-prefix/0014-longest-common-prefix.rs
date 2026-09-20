impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }
        let mut prefix_len = 0;
        // Use 0 as invalid value, since description says only lowercase (97 -> 122)
        const INVALID: u8 = 0;
        'outer: loop {
            let mut ch = INVALID;
            for string in &strs {
                if prefix_len >= string.len() { break 'outer; }
                if ch == INVALID { ch = string.as_bytes()[prefix_len]; }
                else if ch != string.as_bytes()[prefix_len] { break 'outer; }
            }
            prefix_len += 1;
        }
        return strs[0][0..prefix_len].to_string();
    }
}
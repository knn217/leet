impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_len = 0;
        let mut stack = vec![-1]; // Boundary marker

        for (idx, ch) in s.bytes().enumerate() {
            if ch == b'(' {
                stack.push(idx as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    // Reset boundary to current index
                    stack.push(idx as i32);
                } else {
                    // Valid substring length = current index - previous boundary
                    let current_len = idx as i32 - stack.last().unwrap();
                    max_len = max_len.max(current_len);
                }
            }
        }

        max_len
    }
}
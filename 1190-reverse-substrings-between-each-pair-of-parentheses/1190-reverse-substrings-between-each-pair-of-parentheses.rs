impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut pair = vec![0; n];
        let mut stack = Vec::new();

        // Step 1: Pair matching parentheses indices
        for (i, &ch) in s_bytes.iter().enumerate() {
            if ch == b'(' {
                stack.push(i);
            } else if ch == b')' {
                if let Some(open_idx) = stack.pop() {
                    pair[open_idx] = i;
                    pair[i] = open_idx;
                }
            }
        }

        // Step 2: Traverse string with direction switching
        let mut result = String::with_capacity(n);
        let mut curr = 0i32;
        let mut direction = 1i32; // 1 for forward, -1 for backward

        while curr >= 0 && (curr as usize) < n {
            let idx = curr as usize;
            if s_bytes[idx] == b'(' || s_bytes[idx] == b')' {
                curr = pair[idx] as i32; // Teleport to matching paren
                direction = -direction;  // Reverse direction
            } else {
                result.push(s_bytes[idx] as char);
            }
            curr += direction;
        }

        result
    }
}
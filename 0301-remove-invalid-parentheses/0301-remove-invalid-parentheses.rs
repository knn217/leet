use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut found = false;

        queue.push_back(s.clone());
        visited.insert(s);

        while let Some(curr) = queue.pop_front() {
            if Self::is_valid(&curr) {
                result.push(curr.clone());
                found = true;
            }

            // Once a valid level is found, stop expanding to lower levels
            if found {
                continue;
            }

            // Generate all possible states by removing one parenthesis
            for i in 0..curr.len() {
                let ch = curr.as_bytes()[i] as char;
                if ch != '(' && ch != ')' {
                    continue;
                }

                // Create next string by omitting character at index i
                let next_str = format!("{}{}", &curr[..i], &curr[i + 1..]);
                if visited.insert(next_str.clone()) {
                    queue.push_back(next_str);
                }
            }
        }

        result
    }

    // Helper to check if a parentheses string is valid
    fn is_valid(s: &str) -> bool {
        let mut count = 0i32;
        for ch in s.chars() {
            if ch == '(' {
                count += 1;
            } else if ch == ')' {
                count -= 1;
                if count < 0 {
                    return false;
                }
            }
        }
        count == 0
    }
}
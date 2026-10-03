impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() <= 1 { return 0;}
        let mut opening_map: Vec<i32> = vec![-1;s.len()]; // Save the end's openings here
        let mut scope_map: Vec<i32> = vec![-1;s.len() + 1]; // Save the scope's openings here
        let mut scopes = 0;
        for (idx, ch) in s.chars().enumerate() {
            if ch == '(' {
                scopes +=1; // Increment
                scope_map[scopes] = idx as i32; // Save scope opening pos
            }
            if ch == ')' && scopes > 0 {
                let opening = scope_map[scopes]; // Load scope opening pos
                scopes -=1; // decrement
                opening_map[idx] = opening;
            }
        }
        // println!("opening_map: {:?}", opening_map);
        let mut len_max = 0;
        for (end, &opening) in opening_map.iter().enumerate().rev() {
            // println!("opening: {}, end: {}", opening, end);
            // Skip if don't have opening
            if opening == -1 { continue; }
            let mut len_curr = 0;
            let mut end = end;
            let mut opening = opening;
            // Loop to colect all lengths in a valid parenthesis string
            while opening > -1 {
                len_curr += (end - opening as usize + 1);
                let new_end = opening - 1;
                if new_end < 1 { break; }
                // println!("new_end: {}", new_end);
                end = new_end as usize;
                opening = opening_map[end];
            }
            len_max = len_max.max(len_curr);
        }
        len_max as i32
    }
}
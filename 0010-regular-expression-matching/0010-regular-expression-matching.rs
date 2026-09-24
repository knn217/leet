impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let m = s_bytes.len();
        let n = p_bytes.len();

        // dp[j] will track whether s[..i] matches p[..j]
        let mut dp = vec![false; n + 1];

        // Base case: empty string matches empty pattern
        dp[0] = true;

        // Base case: empty string vs patterns with '*' (e.g., "a*", "a*b*")
        for j in 2..=n {
            if p_bytes[j - 1] == b'*' {
                dp[j] = dp[j - 2];
            }
        }

        for i in 1..=m {
            let mut prev_diag = dp[0]; // Stores dp[i-1][j-1]
            dp[0] = false;              // Empty pattern can't match non-empty string

            for j in 1..=n {
                let temp = dp[j]; // Preserve dp[i-1][j] for next iteration's prev_diag

                if p_bytes[j - 1] == b'*' {
                    let zero_occurrences = dp[j - 2];
                    let matches_prev_char = p_bytes[j - 2] == b'.' 
                        || p_bytes[j - 2] == s_bytes[i - 1];
                    
                    let one_or_more = matches_prev_char && dp[j]; // dp[j] is dp[i-1][j]

                    dp[j] = zero_occurrences || one_or_more;
                } else {
                    let matches_curr_char = p_bytes[j - 1] == b'.' 
                        || p_bytes[j - 1] == s_bytes[i - 1];

                    dp[j] = matches_curr_char && prev_diag;
                }

                prev_diag = temp;
            }
        }

        dp[n]
    }
}
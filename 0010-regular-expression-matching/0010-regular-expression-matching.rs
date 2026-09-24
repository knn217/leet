impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let m = s_bytes.len();
        let n = p_bytes.len();

        // dp[i][j] stores whether s[0..i] matches p[0..j]
        let mut dp = vec![vec![false; n + 1]; m + 1];

        // Base case: empty string matches empty pattern
        dp[0][0] = true;

        // Base case: empty string vs patterns with '*' (e.g., "a*", "a*b*")
        for j in 2..=n {
            if p_bytes[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        // Helper closure to match a single character or '.'
        let matches = |i: usize, j: usize| -> bool {
            p_bytes[j - 1] == b'.' || s_bytes[i - 1] == p_bytes[j - 1]
        };

        for i in 1..=m {
            for j in 1..=n {
                if p_bytes[j - 1] == b'*' {
                    // 1. Zero occurrences of preceding character
                    let zero_occurrences = dp[i][j - 2];

                    // 2. One or more occurrences (if current char matches preceding pattern char)
                    let one_or_more = matches(i, j - 1) && dp[i - 1][j];

                    dp[i][j] = zero_occurrences || one_or_more;
                } else {
                    // Regular character or '.' match
                    dp[i][j] = matches(i, j) && dp[i - 1][j - 1];
                }
            }
        }

        dp[m][n]
    }
}
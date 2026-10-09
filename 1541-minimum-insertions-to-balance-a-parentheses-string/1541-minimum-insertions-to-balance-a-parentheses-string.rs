impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut insertions = 0;
        let mut needed_close = 0;
        let mut i = 0;

        while i < n {
            if bytes[i] == b'(' {
                // If needed_close is odd, we have an unmatched single ')'
                // Insert one ')' to pair it before starting a new '('
                if needed_close % 2 != 0 {
                    insertions += 1;
                    needed_close -= 1;
                }
                needed_close += 2;
                i += 1;
            } else {
                // Check if there is a consecutive ')'
                if i + 1 < n && bytes[i + 1] == b')' {
                    i += 2; // Consume both ')'
                } else {
                    insertions += 1; // Insert missing ')'
                    i += 1; // Consume single ')'
                }

                needed_close -= 2;

                // If we don't have enough '(' to pair with these ')'
                if needed_close < 0 {
                    insertions += 1; // Insert one '('
                    needed_close += 2;
                }
            }
        }

        insertions + needed_close
    }
}
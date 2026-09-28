impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_scopes = 0;
        let mut scopes = 0;
        for &ch in s.as_bytes() {
            if ch == b'(' { scopes += 1; }
            if ch == b')' { scopes -= 1; }
            // println!("scopes: {}", scopes);
            max_scopes = max_scopes.max(scopes);
        }
        // println!("max_scopes: {}", max_scopes);
        max_scopes
    }
}
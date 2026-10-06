impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut add = 0;
        let mut scopes = 0;
        for ch in s.chars() {
            if '(' == ch { scopes += 1; }
            if ')' == ch { scopes -= 1; }
            if scopes < 0 {
                scopes = 0;
                add += 1;
            }
        }
        add += scopes;
        add
    }
}
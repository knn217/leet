impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let mut scopes = 0;
        for ch in s.chars() {
            if '(' == ch {
                if scopes > 0 { res.push(ch); }
                scopes += 1;
            }
            if ')' == ch {
                scopes -= 1;
                if scopes > 0 { res.push(ch); }
            }
        }
        res
    }
}
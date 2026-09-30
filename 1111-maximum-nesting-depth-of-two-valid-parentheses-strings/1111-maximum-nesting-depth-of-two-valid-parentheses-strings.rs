impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(seq.len());
        let mut scopes = 0;
        for &ch in seq.as_bytes().iter() {
            if ch == b')' { scopes -= 1; }
            res.push(scopes%2);
            if ch == b'(' { scopes += 1; }
        }
        res
    }
}
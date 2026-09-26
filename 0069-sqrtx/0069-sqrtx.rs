impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 { return 0; }
        
        let mut r = x as i64;
        let target = x as i64;

        while r * r > target {
            r = (r + target / r) / 2;
        }

        r as i32
    }
}
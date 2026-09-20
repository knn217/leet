impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut sum: i32 = 0;
        let mut idx = 1;
        for b in s.as_bytes(){
            sum += ((123 - b) as i32) * idx;
            // println!("{} | {}", 123 - b, sum);
            idx += 1;
        }
        sum
    }
}
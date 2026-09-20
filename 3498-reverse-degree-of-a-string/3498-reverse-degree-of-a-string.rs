impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut sum: i32 = 0;
        for (idx, byte) in s.as_bytes().iter().enumerate() {
            sum += ((123 - byte) as i32) * ((idx + 1) as i32);
            // println!("{} | {}", 123 - byte, sum);
        }
        sum
    }
}
impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut bit_counts = [0; 31];
        for &num in &nums {
            for i in 0..31 {
                if (num & (1 << i)) != 0 {
                    bit_counts[i] += 1;
                }
            }
        }

        let mut total_sum: i64 = 0;
        let modulo: i64 = 1_000_000_007;

        for _ in 0..k {
            let mut curr: i64 = 0;
            for i in (0..31).rev() {
                if bit_counts[i] > 0 {
                    bit_counts[i] -= 1;
                    curr |= 1 << i;
                }
            }
            total_sum = (total_sum + curr * curr) % modulo;
        }

        total_sum as i32
    }
}
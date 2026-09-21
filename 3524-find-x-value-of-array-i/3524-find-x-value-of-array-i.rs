impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let k_usize = k as usize;
        let mut total_counts = vec![0i64; k_usize];
        // dp[r] stores the number of valid subarrays ending at the current position where the product modulo k equals r.
        let mut dp = vec![0i64; k_usize];
        for &num in &nums {
            let mut next_dp = vec![0i64; k_usize];
            let val = (num % k) as usize;
            // Option 1: Start a new contiguous subarray with just `num`
            next_dp[val] += 1;
            // Option 2: Extend all contiguous subarrays ending at the previous element
            for prev_rem in 0..k_usize {
                if dp[prev_rem] > 0 {
                    let new_rem = (prev_rem * val) % k_usize;
                    next_dp[new_rem] += dp[prev_rem];
                }
            }
            // Accumulate into total counts and update dp state
            for r in 0..k_usize {
                total_counts[r] += next_dp[r];
            }
            dp = next_dp;
        }
        total_counts
    }
}
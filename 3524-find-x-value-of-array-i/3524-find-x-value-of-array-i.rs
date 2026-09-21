impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let nums_len = nums.len() as i64;
        let k = k as usize;
        let mut res = vec![0i64; k];
        res[0] = nums_len * (nums_len + 1) / 2;
        // map[r] stores the number of valid subarrays ending at the current position where the product modulo k equals r.
        let mut map = vec![0i64; k];
        for &num in &nums {
            let mut next_map = vec![0i64; k];
            let remainder = num as usize % k;
            // Option 1: Start a new contiguous subarray with just `num`
            next_map[remainder] += 1;
            // Option 2: Extend all contiguous subarrays ending at the previous element
            for prev_remainder in 0..k {
                if map[prev_remainder] <= 0 { continue; }
                let new_remainder = (prev_remainder * remainder) % k;
                next_map[new_remainder] += map[prev_remainder];
            }
            // Accumulate into total counts and update map state
            for r in 1..k {
                res[0] -= next_map[r];
                res[r] += next_map[r];
            }
            map = next_map;
        }
        res
    }
}

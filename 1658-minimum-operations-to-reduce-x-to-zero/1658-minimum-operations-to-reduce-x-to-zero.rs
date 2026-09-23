impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let total_sum: i64 = nums.iter().map(|&n| n as i64).sum();
        let target = total_sum - x as i64;

        // If target is 0, we must consume the entire array
        if target == 0 {
            return nums.len() as i32;
        }
        // If target < 0, total_sum < x, impossible to reach 0
        if target < 0 {
            return -1;
        }

        let mut current_sum: i64 = 0;
        let mut max_len: i32 = -1;
        let mut left = 0;

        for right in 0..nums.len() {
            current_sum += nums[right] as i64;

            // Shrink window from the left if sum exceeds target
            while current_sum > target && left <= right {
                current_sum -= nums[left] as i64;
                left += 1;
            }

            // Check if we found a valid middle subarray
            if current_sum == target {
                max_len = max_len.max((right - left + 1) as i32);
            }
        }

        if max_len == -1 {
            -1
        } else {
            nums.len() as i32 - max_len
        }
    }
}
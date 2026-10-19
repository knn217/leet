impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut count = 0;
        let mut smallest_idx = 0;
        for num in nums {
            if num == target { count += 1; }
            if num < target { smallest_idx += 1; }
        }
        let mut res: Vec<i32> = Vec::with_capacity(count);
        for i in 0..count {
            res.push((i + smallest_idx) as i32);
        }
        res
    }
}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // Edge case
        if nums.len() <= 1 { return; }
        // Normal case
        let mut idx_changing = nums.len();
        for (idx, &num) in nums.iter().enumerate().rev().skip(1) {
            if num < nums[idx + 1] {
                idx_changing = idx;
                break;
            }
        }
        // println!("idx_changing: {}", idx_changing);
        if idx_changing == nums.len() {
            nums.sort();
        } else {
            let mut idx_greater = nums.len() - 1;
            for (idx, &num) in nums.iter().enumerate().rev() {
                if num > nums[idx_changing] {
                    idx_greater = idx;
                    break;
                }
            }
            // println!("idx_greater: {}", idx_greater);
            if idx_greater < 0 { return; }
            nums.swap(idx_changing, idx_greater as usize);
            nums[(idx_changing+1)..].sort();
        }
        return;
    }
}
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        // Edge case: only 1 answer
        if nums.len() == 3 {
            return nums.iter().sum();
        }
        //  normal cases
        let mut nums = nums;
        nums.sort();
        let mut sum_closest: Option<i32> = None;
        for (idx_i, &val_i) in nums.iter().enumerate() {
            if idx_i > (nums.len() - 2) { break; }
            let mut idx_j = idx_i + 1;
            let mut idx_k = nums.len() - 1;
            let mut sum = i32::MAX;
            while idx_j < idx_k {
                if !(0..nums.len()).contains(&idx_j) { break; }
                if !(0..nums.len()).contains(&idx_k) { break; }
                let val_j = nums[idx_j];
                let val_k = nums[idx_k];
                sum = val_i + val_j + val_k;
                // println!("idx_i: {}, idx_j: {}, idx_k: {}", idx_i, idx_j, idx_k);
                // println!("val_i: {}, val_j: {}, val_k: {}", val_i, val_j, val_k);
                // println!("sum: {}", sum);
                if sum == target { return sum; }
                else if sum < target { idx_j += 1; }
                else if sum > target { idx_k -= 1; }
                if (None == sum_closest) {
                    sum_closest = Some(sum);
                }
                if (target - sum).abs() < (target - sum_closest.unwrap()).abs() {
                    sum_closest = Some(sum);
                }
                // println!("sum_closest: {}", sum_closest.unwrap());
            }
        }
        sum_closest.unwrap()
    }
}
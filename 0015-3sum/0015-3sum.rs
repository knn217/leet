impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable(); // O(N*log(N))
        // println!("nums: {:?}", nums);
        let mut res: Vec<Vec<i32>> = vec![];
        for (idx_i, val_i) in nums.iter().enumerate() {
            // Break if val_i > 0 since val_j, val_k > val_i
            if *val_i > 0 { break; }
            // Continue if val_i was repeated
            if idx_i > 0 && nums[idx_i - 1] == *val_i { continue; }
            let mut idx_j = idx_i + 1;
            let mut idx_k = nums.len() - 1;
            while idx_j < idx_k {
                let val_j = nums[idx_j];
                let val_k = nums[idx_k];
                let sum = val_i + val_j + val_k;
                if sum == 0 {
                    res.push(Vec::from([*val_i, val_j, val_k]));
                    // println!("idx_i: {}, idx_j: {}, idx_k: {}", idx_i, idx_j, idx_k);
                    // println!("val_i: {}, val_j: {}, val_k: {}", val_i, val_j, val_k);
                    // println!("res: {:?}", res);
                    while idx_j < idx_k && val_j == nums[idx_j] { idx_j += 1; }
                    while idx_j < idx_k && val_k == nums[idx_k] { idx_k -= 1; }
                }
                else if sum < 0 { idx_j += 1; }
                else if sum > 0 { idx_k -= 1; }
                if val_k <= 0 { break; }
            }
        }
        res
    }
}
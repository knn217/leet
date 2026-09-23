impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 3 {
            return vec![];
        }

        nums.sort_unstable(); // Slower than sort() for tiny slices, but skips stability guarantees

        let mut res = Vec::new();

        for i in 0..n - 2 {
            let val_i = nums[i];

            // Minimal sum of remaining elements is already > 0
            if val_i > 0 {
                break;
            }

            // Skip duplicates for first element
            if i > 0 && nums[i - 1] == val_i {
                continue;
            }

            let mut j = i + 1;
            let mut k = n - 1;

            while j < k {
                let sum = val_i + nums[j] + nums[k];

                if sum == 0 {
                    res.push(vec![val_i, nums[j], nums[k]]);

                    let val_j = nums[j];
                    let val_k = nums[k];

                    while j < k && nums[j] == val_j {
                        j += 1;
                    }
                    while j < k && nums[k] == val_k {
                        k -= 1;
                    }
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        res
    }
}
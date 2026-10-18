impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();

        // Precompute combinations using Pascal's triangle
        let mut comb = vec![vec![0i64; n + 1]; n + 1];
        for i in 0..=n {
            comb[i][0] = 1;
            for j in 1..=i {
                comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD;
            }
        }

        fn solve(nums: &[i32], comb: &Vec<Vec<i64>>, mod_val: i64) -> i64 {
            if nums.len() <= 2 {
                return 1;
            }

            let root = nums[0];
            let mut left = Vec::new();
            let mut right = Vec::new();

            for &x in nums.iter().skip(1) {
                if x < root {
                    left.push(x);
                } else {
                    right.push(x);
                }
            }

            let left_ways = solve(&left, comb, mod_val);
            let right_ways = solve(&right, comb, mod_val);

            let l_len = left.len();
            let r_len = right.len();

            // Total ways to interleave left and right subtree sequences
            let ways = comb[l_len + r_len][l_len];

            // Multiply combination choices with the number of ways for left and right subtrees
            (((ways * left_ways) % mod_val) * right_ways) % mod_val
        }

        // Subtract 1 to exclude the original array order
        ((solve(&nums, &comb, MOD) - 1 + MOD) % MOD) as i32
    }
}
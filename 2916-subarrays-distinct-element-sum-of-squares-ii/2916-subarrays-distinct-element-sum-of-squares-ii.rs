impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        const MOD: i64 = 1_000_000_007;

        #[derive(Clone, Copy)]
        struct Node {
            sum: i64,
            sum_sq: i64,
            lazy: i64,
        }

        let mut tree = vec![Node { sum: 0, sum_sq: 0, lazy: 0 }; 4 * n];

        fn push(node: usize, l: usize, r: usize, tree: &mut [Node], mod_val: i64) {
            if tree[node].lazy != 0 {
                let lazy = tree[node].lazy;
                let mid = (l + r) / 2;

                // Left child
                let left = 2 * node;
                let len_left = (mid - l + 1) as i64;
                tree[left].sum_sq = (tree[left].sum_sq + 2 * lazy * tree[left].sum + lazy * lazy % mod_val * len_left) % mod_val;
                tree[left].sum = (tree[left].sum + lazy * len_left) % mod_val;
                tree[left].lazy = (tree[left].lazy + lazy) % mod_val;

                // Right child
                let right = 2 * node + 1;
                let len_right = (r - mid) as i64;
                tree[right].sum_sq = (tree[right].sum_sq + 2 * lazy * tree[right].sum + lazy * lazy % mod_val * len_right) % mod_val;
                tree[right].sum = (tree[right].sum + lazy * len_right) % mod_val;
                tree[right].lazy = (tree[right].lazy + lazy) % mod_val;

                tree[node].lazy = 0;
            }
        }

        fn update(node: usize, l: usize, r: usize, ql: usize, qr: usize, tree: &mut [Node], mod_val: i64) {
            if ql <= l && r <= qr {
                let len = (r - l + 1) as i64;
                tree[node].sum_sq = (tree[node].sum_sq + 2 * tree[node].sum + len) % mod_val;
                tree[node].sum = (tree[node].sum + len) % mod_val;
                tree[node].lazy = (tree[node].lazy + 1) % mod_val;
                return;
            }
            push(node, l, r, tree, mod_val);
            let mid = (l + r) / 2;
            if ql <= mid {
                update(2 * node, l, mid, ql, qr, tree, mod_val);
            }
            if qr > mid {
                update(2 * node + 1, mid + 1, r, ql, qr, tree, mod_val);
            }
            tree[node].sum = (tree[2 * node].sum + tree[2 * node + 1].sum) % mod_val;
            tree[node].sum_sq = (tree[2 * node].sum_sq + tree[2 * node + 1].sum_sq) % mod_val;
        }

        let mut last = std::collections::HashMap::new();
        let mut total_sum_sq = 0i64;

        for r in 0..n {
            let l = match last.get(&nums[r]) {
                Some(&prev) => prev + 1,
                None => 0,
            };
            last.insert(nums[r], r);
            update(1, 0, n - 1, l, r, &mut tree, MOD);
            total_sum_sq = (total_sum_sq + tree[1].sum_sq) % MOD;
        }

        total_sum_sq as i32
    }
}
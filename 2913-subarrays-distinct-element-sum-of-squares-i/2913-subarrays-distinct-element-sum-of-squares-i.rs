const MOD: i64 = 1_000_000_007;

#[derive(Clone)]
struct Node {
    sum_sq: i64,
    sum_c: i64,
    lazy: i64,
}

struct SegmentTree {
    tree: Vec<Node>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        SegmentTree {
            tree: vec![Node { sum_sq: 0, sum_c: 0, lazy: 0 }; 4 * n],
        }
    }

    fn push(&mut self, node: usize, l: usize, r: usize) {
        let lazy = self.tree[node].lazy;
        if lazy != 0 {
            let mid = l + (r - l) / 2;
            self.apply(2 * node, l, mid, lazy);
            self.apply(2 * node + 1, mid + 1, r, lazy);
            self.tree[node].lazy = 0;
        }
    }

    fn apply(&mut self, node: usize, l: usize, r: usize, val: i64) {
        let len = (r - l + 1) as i64;
        let sum_sq = self.tree[node].sum_sq;
        let sum_c = self.tree[node].sum_c;
        
        self.tree[node].sum_sq = (sum_sq + 2 * val * sum_c + val * val * len) % MOD;
        self.tree[node].sum_c = (sum_c + val * len) % MOD;
        self.tree[node].lazy = (self.tree[node].lazy + val) % MOD;
    }

    fn update(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, val: i64) {
        if ql > r || qr < l {
            return;
        }
        if ql <= l && r <= qr {
            self.apply(node, l, r, val);
            return;
        }
        self.push(node, l, r);
        let mid = l + (r - l) / 2;
        self.update(2 * node, l, mid, ql, qr, val);
        self.update(2 * node + 1, mid + 1, r, ql, qr, val);
        
        self.tree[node].sum_sq = (self.tree[2 * node].sum_sq + self.tree[2 * node + 1].sum_sq) % MOD;
        self.tree[node].sum_c = (self.tree[2 * node].sum_c + self.tree[2 * node + 1].sum_c) % MOD;
    }

    fn query_sum_sq(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql > r || qr < l {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.tree[node].sum_sq;
        }
        self.push(node, l, r);
        let mid = l + (r - l) / 2;
        let left_res = self.query_sum_sq(2 * node, l, mid, ql, qr);
        let right_res = self.query_sum_sq(2 * node + 1, mid + 1, r, ql, qr);
        (left_res + right_res) % MOD
    }
}

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut st = SegmentTree::new(n);
        let mut last = std::collections::HashMap::new();
        let mut total_sum = 0i64;

        for i in 0..n {
            let prev = *last.get(&nums[i]).unwrap_or(&-1) as usize;
            let l = prev + 1;
            let r = i;
            
            st.update(1, 0, n - 1, l, r, 1);
            last.insert(nums[i], i as i32);
            
            let current_total = st.query_sum_sq(1, 0, n - 1, 0, i);
            total_sum = (total_sum + current_total) % MOD;
        }

        total_sum as i32
    }
}
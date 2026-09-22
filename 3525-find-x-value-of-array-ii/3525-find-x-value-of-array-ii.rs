struct SegmentTree {
    n: usize,
    k: i32,
    // tree[i].dp[r] = count of prefixes in this segment with product % k == r
    // tree[i].prod = product of all elements in this segment % k
    dp: Vec<Vec<i32>>,
    prod: Vec<i32>,
}

impl SegmentTree {
    fn new(nums: &[i32], k: i32) -> Self {
        let n = nums.len();
        let mut st = SegmentTree {
            n,
            k,
            dp: vec![vec![0; k as usize]; 4 * n],
            prod: vec![1; 4 * n],
        };
        st.build(nums, 1, 0, n - 1);
        st
    }

    fn merge_nodes(&mut self, node: usize, left: usize, right: usize) {
        let k = self.k as usize;
        let l_prod = self.prod[left] as usize;

        // Reset current node
        self.prod[node] = ((self.prod[left] as i64 * self.prod[right] as i64) % self.k as i64) as i32;
        self.dp[node].fill(0);

        // 1. Prefixes entirely within the left child
        for r in 0..k {
            self.dp[node][r] += self.dp[left][r];
        }

        // 2. Prefixes starting at left child and extending into right child
        for r_right in 0..k {
            let count = self.dp[right][r_right];
            if count > 0 {
                let combined_rem = (l_prod * r_right) % k;
                self.dp[node][combined_rem] += count;
            }
        }
    }

    fn build(&mut self, nums: &[i32], node: usize, start: usize, end: usize) {
        if start == end {
            let val = (nums[start] % self.k) as usize;
            self.prod[node] = val as i32;
            self.dp[node][val] = 1;
            return;
        }

        let mid = (start + end) / 2;
        self.build(nums, 2 * node, start, mid);
        self.build(nums, 2 * node + 1, mid + 1, end);
        self.merge_nodes(node, 2 * node, 2 * node + 1);
    }

    pub fn update(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        if start == end {
            let rem = (val % self.k) as usize;
            self.prod[node] = rem as i32;
            self.dp[node].fill(0);
            self.dp[node][rem] = 1;
            return;
        }

        let mid = (start + end) / 2;
        if idx <= mid {
            self.update(2 * node, start, mid, idx, val);
        } else {
            self.update(2 * node + 1, mid + 1, end, idx, val);
        }
        self.merge_nodes(node, 2 * node, 2 * node + 1);
    }

    // Query prefix counts for subarray [query_start..n-1]
    pub fn query_prefix(&self, node: usize, start: usize, end: usize, query_start: usize) -> (Vec<i32>, i32) {
        if start >= query_start {
            return (self.dp[node].clone(), self.prod[node]);
        }

        let mid = (start + end) / 2;
        let k = self.k as usize;

        if query_start > mid {
            return self.query_prefix(2 * node + 1, mid + 1, end, query_start);
        }

        let (left_dp, left_prod) = self.query_prefix(2 * node, start, mid, query_start);
        let (right_dp, right_prod) = self.query_prefix(2 * node + 1, mid + 1, end, query_start);

        let mut res_dp = vec![0; k];
        let total_prod = ((left_prod as i64 * right_prod as i64) % self.k as i64) as i32;

        // Prefixes ending inside the left segment
        for r in 0..k {
            res_dp[r] += left_dp[r];
        }

        // Prefixes spanning across into the right segment
        for r_right in 0..k {
            let count = right_dp[r_right];
            if count > 0 {
                let combined_rem = ((left_prod as usize) * r_right) % k;
                res_dp[combined_rem] += count;
            }
        }

        (res_dp, total_prod)
    }
}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut tree = SegmentTree::new(&nums, k);
        let mut ans = Vec::with_capacity(queries.len());

        for q in queries {
            let idx = q[0] as usize;
            let val = q[1];
            let start_idx = q[2] as usize;
            let target_x = q[3] as usize;

            // Point update
            tree.update(1, 0, n - 1, idx, val);

            // Range query
            let (dp, _) = tree.query_prefix(1, 0, n - 1, start_idx);
            ans.push(dp[target_x]);
        }

        ans
    }
}
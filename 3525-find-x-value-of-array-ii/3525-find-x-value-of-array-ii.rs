#[derive(Clone, Copy)]
struct Node {
    dp: [i32; 10],
    prod: i32,
}

impl Node {
    const fn empty() -> Self {
        Self {
            dp: [0; 10],
            prod: 1,
        }
    }
}

struct ZeroAllocSegTree {
    n: usize,
    k: usize,
    tree: Vec<Node>, // Single pre-allocation
}

impl ZeroAllocSegTree {
    pub fn new(nums: &[i32], k: i32) -> Self {
        let n = nums.len();
        let k_usize = k as usize;
        let mut st = ZeroAllocSegTree {
            n,
            k: k_usize,
            tree: vec![Node::empty(); 4 * n],
        };
        st.build(nums, 1, 0, n - 1);
        st
    }

    #[inline(always)]
    fn merge(left: &Node, right: &Node, k: usize) -> Node {
        let mut parent = Node::empty();
        
        // Combine range products
        parent.prod = ((left.prod as i64 * right.prod as i64) % k as i64) as i32;

        // 1. Prefixes entirely within the left child
        for r in 0..k {
            parent.dp[r] += left.dp[r];
        }

        // 2. Prefixes spanning across left child into right child
        let l_prod = left.prod as usize;
        for r_right in 0..k {
            let count = right.dp[r_right];
            if count > 0 {
                let combined_rem = (l_prod * r_right) % k;
                parent.dp[combined_rem] += count;
            }
        }

        parent
    }

    fn build(&mut self, nums: &[i32], node: usize, start: usize, end: usize) {
        if start == end {
            let val = (nums[start] as usize) % self.k;
            self.tree[node].prod = val as i32;
            self.tree[node].dp[val] = 1;
            return;
        }

        let mid = (start + end) / 2;
        let left_child = 2 * node;
        let right_child = 2 * node + 1;

        self.build(nums, left_child, start, mid);
        self.build(nums, right_child, mid + 1, end);

        self.tree[node] = Self::merge(&self.tree[left_child], &self.tree[right_child], self.k);
    }

    pub fn update(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        if start == end {
            let rem = (val as usize) % self.k;
            self.tree[node].prod = rem as i32;
            self.tree[node].dp = [0; 10];
            self.tree[node].dp[rem] = 1;
            return;
        }

        let mid = (start + end) / 2;
        let left_child = 2 * node;
        let right_child = 2 * node + 1;

        if idx <= mid {
            self.update(left_child, start, mid, idx, val);
        } else {
            self.update(right_child, mid + 1, end, idx, val);
        }

        self.tree[node] = Self::merge(&self.tree[left_child], &self.tree[right_child], self.k);
    }

    // Query prefix counts with zero allocations (stack-passed Node struct)
    pub fn query(&self, node: usize, start: usize, end: usize, query_start: usize) -> Node {
        if start >= query_start {
            return self.tree[node];
        }

        let mid = (start + end) / 2;
        if query_start > mid {
            return self.query(2 * node + 1, mid + 1, end, query_start);
        }

        let left_node = self.query(2 * node, start, mid, query_start);
        let right_node = self.query(2 * node + 1, mid + 1, end, query_start);

        Self::merge(&left_node, &right_node, self.k)
    }
}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut tree = ZeroAllocSegTree::new(&nums, k);
        
        // Pre-allocate answer vector to size Q
        let mut ans = Vec::with_capacity(queries.len());

        for q in queries {
            let idx = q[0] as usize;
            let val = q[1];
            let start_idx = q[2] as usize;
            let target_x = q[3] as usize;

            // Point update: O(K log N), 0 allocations
            tree.update(1, 0, n - 1, idx, val);

            // Range query: O(K log N), 0 allocations
            let res_node = tree.query(1, 0, n - 1, start_idx);
            ans.push(res_node.dp[target_x]);
        }

        ans
    }
}
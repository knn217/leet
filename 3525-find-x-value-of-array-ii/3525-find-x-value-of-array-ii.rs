// Size: MAX_K * 4 bytes + 4 bytes -> packed tightly
const MAX_K: usize = 5;

#[derive(Clone, Copy)]
struct Node {
    dp: [i32; MAX_K],
    prod: i8, // Since K <= MAX_K, i8 fits [0..MAX_K] easily
}

impl Node {
    const fn empty() -> Self {
        Self {
            dp: [0; MAX_K],
            prod: 1,
        }
    }
}

struct CompactSegTree {
    n: usize, // Store num's actual length
    k: usize, // Store the modulus
    tree: Vec<Node>, // Allocated exactly 2 * N (NOT 4 * N!)
}

impl CompactSegTree {
    pub fn new(nums: &[i32], k: i32) -> Self {
        let n = nums.len();
        let k_usize = k as usize;
        
        // Exact 2 * N memory allocation, exact theoretical minimum for all leaf nodes + parent nodes
        let mut tree = vec![Node::empty(); 2 * n];

        // Build leaf nodes
        for i in 0..n {
            let rem = (nums[i] as usize) % k_usize;
            tree[n + i].prod = rem as i8;
            tree[n + i].dp[rem] = 1;
        }

        // Build parent nodes bottom-up
        for i in (1..n).rev() {
            tree[i] = Self::merge(&tree[2 * i], &tree[2 * i + 1], k_usize);
        }

        CompactSegTree { n, k: k_usize, tree }
    }

    #[inline(always)]
    fn merge(left: &Node, right: &Node, k: usize) -> Node {
        let mut parent = Node::empty();
        
        // Product fits in i8
        parent.prod = ((left.prod as i16 * right.prod as i16) % (k as i16)) as i8;

        // Prefixes from left
        for r in 0..k {
            parent.dp[r] += left.dp[r];
        }

        // Prefixes extending into right
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

    pub fn update(&mut self, mut idx: usize, val: i32) {
        idx += self.n;
        let rem = (val as usize) % self.k;
        self.tree[idx].prod = rem as i8;
        self.tree[idx].dp = [0; MAX_K];
        self.tree[idx].dp[rem] = 1;

        // Propagate up
        while idx > 1 {
            idx /= 2;
            self.tree[idx] = Self::merge(&self.tree[2 * idx], &self.tree[2 * idx + 1], self.k);
        }
    }

    pub fn query(&self, mut left: usize, mut right: usize) -> Node {
        left += self.n;
        right += self.n;

        let mut left_res = Node::empty();
        let mut right_res = Node::empty();
        let mut has_left = false;
        let mut has_right = false;

        while left <= right {
            if left % 2 == 1 {
                if !has_left {
                    left_res = self.tree[left];
                    has_left = true;
                } else {
                    left_res = Self::merge(&left_res, &self.tree[left], self.k);
                }
                left += 1;
            }
            if right % 2 == 0 {
                if !has_right {
                    right_res = self.tree[right];
                    has_right = true;
                } else {
                    right_res = Self::merge(&self.tree[right], &right_res, self.k);
                }
                right -= 1;
            }
            left /= 2;
            right /= 2;
        }

        if !has_left {
            right_res
        } else if !has_right {
            left_res
        } else {
            Self::merge(&left_res, &right_res, self.k)
        }
    }
}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut tree = CompactSegTree::new(&nums, k);
        let mut ans = Vec::with_capacity(queries.len());

        for q in queries {
            let idx = q[0] as usize;
            let val = q[1];
            let start_idx = q[2] as usize;
            let target_x = q[3] as usize;

            tree.update(idx, val);
            let res_node = tree.query(start_idx, n - 1);
            ans.push(res_node.dp[target_x]);
        }

        ans
    }
}
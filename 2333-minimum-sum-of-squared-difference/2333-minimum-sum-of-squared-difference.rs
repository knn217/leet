impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let n = nums1.len();
        let mut diff: Vec<i64> = Vec::with_capacity(n);
        let total_k = (k1 as i64) + (k2 as i64);
        
        for i in 0..n {
            diff.push((nums1[i] as i64 - nums2[i] as i64).abs());
        }
        
        let sum_diff: i64 = diff.iter().sum();
        if sum_diff <= total_k {
            return 0;
        }
        
        let mut low = 0;
        let mut high = *diff.iter().max().unwrap();
        let mut ans = high;
        
        while low <= high {
            let mid = low + (high - low) / 2;
            let operations: i64 = diff.iter().map(|&d| if d > mid { d - mid } else { 0 }).sum();
            if operations <= total_k {
                ans = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        
        let mut remaining = total_k;
        for d in diff.iter_mut() {
            if *d > ans {
                remaining -= *d - ans;
                *d = ans;
            }
        }
        
        for d in diff.iter_mut() {
            if remaining > 0 && *d == ans {
                *d -= 1;
                remaining -= 1;
            }
        }
        
        diff.iter().map(|&d| d * d).sum()
    }
}
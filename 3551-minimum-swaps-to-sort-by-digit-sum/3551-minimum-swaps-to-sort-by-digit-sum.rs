impl Solution {
    fn digit_sum(mut x: i32) -> i32 {
        let mut sum = 0;
        while x > 0 {
            sum += x % 10;
            x /= 10;
        }
        sum
    }

    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        // Helper function to calculate the digit sum of a number
        
        // Create tuples of (digit_sum, value, original_index)
        let mut arr: Vec<(i32, i32, usize)> = nums
            .iter()
            .enumerate()
            .map(|(i, &val)| (Self::digit_sum(val), val, i))
            .collect();
            
        // Sort by digit sum first, then by numeric value
        arr.sort_unstable_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        
        // Find cycles to calculate minimum swaps
        let mut visited = vec![false; n];
        let mut swaps = 0;
        
        for i in 0..n {
            if visited[i] || arr[i].2 == i {
                continue;
            }
            
            let mut cycle_size = 0;
            let mut curr = i;
            while !visited[curr] {
                visited[curr] = true;
                curr = arr[curr].2;
                cycle_size += 1;
            }
            
            if cycle_size > 1 {
                swaps += cycle_size - 1;
            }
        }
        
        swaps as i32
    }
}
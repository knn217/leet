impl Solution {
    fn bin_search(nums: &Vec<i32>, target: &i32, low: usize, high: usize) -> i32 {
        let mut low = low;
        let mut high = high;
        let mut mid = (low + high) / 2;
        loop {
            match *target {
                n if nums[low] == n => { return low as i32; }
                n if nums[high] == n => { return high as i32; }
                n if nums[mid] == n => { return mid as i32; }
                n if nums[low] > n => { return -1; }
                n if nums[high] < n => { return -1; }
                n if nums[low] < n && n < nums[mid] => {
                    if low + 1 == mid { return -1; }
                    high = mid;
                    mid = (low + high) / 2;
                }
                n if nums[mid] < n && n < nums[high] => {
                    if mid + 1 == high { return -1; }
                    low = mid;
                    mid = (low + high) / 2;
                }
                _ => {}
            }
        }
        -1
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // O(N*log(N))
        // println!("nums: {:?}", nums);
        let mut res: Vec<Vec<i32>> = vec![];
        for (idx_i, val_i) in nums.iter().enumerate() {
            if idx_i + 2 >= nums.len() { break; }
            if idx_i > 0 && nums[idx_i - 1] == *val_i { continue; }
            let val_jk = 0 - val_i;
            for (idx_j, val_j) in nums.iter().enumerate().skip(idx_i + 1) {
                if idx_j + 1 >= nums.len() { break; }
                if (idx_j - idx_i) > 1 && nums[idx_j - 1] == *val_j { continue; }
                let val_k = val_jk - val_j;
                let idx_k = Self::bin_search(&nums, &val_k, idx_j+1, nums.len()-1);
                // println!("idx_i: {}, idx_j: {}, idx_k: {}", idx_i, idx_j, idx_k);
                // println!("val_i: {}, val_j: {}, val_k: {}", val_i, val_j, val_k);
                if idx_k < 0 { continue; }
                if nums[idx_k as usize] == val_k {
                    res.push(Vec::from([*val_i, *val_j, val_k]));
                }
                // println!("res: {:?}", res);
            }
        }
        res
    }
}
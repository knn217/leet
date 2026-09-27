impl Solution {
    fn bin_search(nums: &[i32], target: f32) -> i32 {
        // if target exist, return lowest index with same value
        // if target not exist, return index to insert
        if nums.len() == 0 { return -1; }
        if nums.len() == 1 && nums[0] as f32 <= target { return 0; }
        if nums.len() == 1 && nums[0] as f32 > target { return 1; }
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut mid = (high + low) / 2;
        loop {
            match target {
                n if (n + 0.5) == (nums[low] as f32) => { return low as i32; }
                n if (n - 0.5) == (nums[high] as f32) => { return high as i32 + 1; }
                n if n < nums[low] as f32 => { return -1; }
                n if n > nums[high] as f32 => { return -1; }
                n if n > nums[low] as f32 && n < nums[mid] as f32 => {
                    if (low+1) == mid { return mid as i32; }
                    high = mid;
                    mid = (high + low) / 2;
                }
                n if n > nums[mid] as f32 && n < nums[high] as f32 => {
                    if (mid+1) == high { return high as i32; }
                    low = mid;
                    mid = (high + low) / 2;
                }
                _=>{}
            }
        }
        -1
    }
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 { return Vec::from([-1, -1]); }
        if nums.len() == 1 {
            if nums[0] == target { return Vec::from([0, 0]); }
            else { return Vec::from([-1, -1]); }
        }
        let mut res: Vec<i32> = vec![];
        let mut start = Self::bin_search(&nums, target as f32 - 0.5);
        let mut end = Self::bin_search(&nums, target as f32 + 0.5);
        if (end > 0) {end -= 1;}
        // println!("start: {}, end: {}", start, end);
        // Validate the indices
        if !(0..nums.len()).contains(&(start as usize)) { start = -1; }
        if !(0..nums.len()).contains(&(end as usize)) { end = -1; }
        if (start >= 0) && (target != nums[start as usize]) { start = -1; }
        if (end >= 0) && (target != nums[end as usize]) { end = -1; }
        // println!("start: {}, end: {}", start, end);
        res.push(start);
        res.push(end);
        res
    }
}
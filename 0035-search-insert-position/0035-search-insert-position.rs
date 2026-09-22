impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut mid = (low + high) / 2;
        loop {
            match target {
                n if nums[low] == n => { return low as i32; }
                n if nums[high] == n => { return high as i32; }
                n if nums[mid] == n => { return mid as i32; }
                n if nums[low] > n => { return low as i32; }
                n if nums[high] < n => { return (high + 1) as i32; }
                n if nums[low] < n && n < nums[mid] => {
                    // println!("between low {} and mid {}", low, mid);
                    if low + 1 == mid { return mid as i32; }
                    high = mid;
                    mid = (low + high) / 2;
                }
                n if nums[mid] < n && n < nums[high] => {
                    // println!("between mid {} and high {}", mid, high);
                    if mid + 1 == high { return high as i32; }
                    low = mid;
                    mid = (low + high) / 2;
                }
                _ => {}
            }
        }
        -1
    }
}
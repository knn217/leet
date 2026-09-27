impl Solution {
    fn bin_search(nums: &[i32], target: &i32) -> i32 {
        if nums.len() == 0 { return -1; }
        if nums.len() == 1 && nums[0] == *target { return 0; }
        // println!("Binary searching in {:?}", nums);
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut mid = (high + low) / 2;
        loop{
            match *target {
                n if n == nums[low] => {return low as i32;}
                n if n == nums[mid] => {return mid as i32;}
                n if n == nums[high] => {return high as i32;}
                n if n < nums[low] => {return -1;}
                n if n > nums[high] => {return -1;}
                n if n > nums[low] && n < nums[mid] => {
                    if (low+1) == mid { return -1; }
                    high = mid;
                    mid = (high + low) / 2;
                }
                n if n > nums[mid] && n < nums[high] => {
                    if (mid+1) == high { return -1; }
                    low = mid;
                    mid = (high + low) / 2;
                }
                _ => {return -1;}
            }
        }
        -1
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 { return -1; }
        if nums.len() == 1 && nums[0] == target { return 0; }
        let rotated = (nums.first() > nums.last());
        // Find the rotated point in O(log(N))
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut test = (high + low) / 2;
        let mut rotation = 0;
        if rotated {
            loop {
                if nums[test] > nums[test+1] {
                    // Found rotation
                    // println!("Found rotation at {}", test+1);
                    rotation = test+1;
                    break;
                }
                else if nums[test] > nums[low] {
                    // test is in the lower half, shift the range up
                    low = test;
                    test = (high + low) / 2;
                }
                else if nums[test] < nums[high] {
                    // test is in the upper half, shift the range down
                    high = test;
                    test = (high + low) / 2;
                }
            }
        }
        let mut left_search = Self::bin_search(&nums[..rotation], &target);
        let mut right_search = Self::bin_search(&nums[rotation..], &target);
        // println!("left_search: {}, right_search: {}", left_search, right_search);
        if right_search != -1 {
            right_search += rotation as i32;
        }
        // println!("left_search: {}, right_search: {}", left_search, right_search);
        left_search.max(right_search)
    }
}
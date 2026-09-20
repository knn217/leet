impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut vec: Vec<i32> = Vec::with_capacity(nums.len());
        // Use this value since numbers are in range [-100, 100]
        let mut current = -101;
        for num in nums.iter() {
            if *num != current {
                current = *num;
                vec.push(*num);
            }
        }
        *nums = vec;
        nums.len() as i32
    }
}
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0;
        let n = nums.len();

        for i in 0..n {
            let mut distinct = std::collections::HashSet::new();
            for j in i..n {
                distinct.insert(nums[j]);
                let count = distinct.len() as i32;
                total_sum += count * count;
            }
        }

        total_sum
    }
}
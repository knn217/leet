impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        for (idx, num) in nums.iter().enumerate() {
            if 0 == (n % (idx+1)) { sum += num*num; }
        }
        sum
    }
}
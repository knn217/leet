impl Solution {
    fn sum_digit(num: i32) -> i32 {
        let mut num = num;
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (idx, num) in nums.iter().enumerate() {
            let idx = idx as i32;
            if idx == Self::sum_digit(*num) { return idx; }
        }
        -1
    }
}
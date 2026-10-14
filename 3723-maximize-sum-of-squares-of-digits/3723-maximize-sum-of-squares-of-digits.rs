impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        let mut s = String::with_capacity(num as usize);
        let num = num as usize;
        let mut sum = sum;
        for digit in 0..num {
            if sum <= 0 {
                s.push_str(&("0".repeat(num - digit)));
                return s;
            }
            let min_digit = sum.min(9);
            s.push_str(&(min_digit.to_string()));
            sum -= min_digit;
        }
        if sum > 0 { return String::from(""); }
        s
    }
}
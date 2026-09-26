impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut increment = 1;
        for digit in digits.iter_mut().rev() {
            *digit += increment;
            increment = *digit / 10;
            *digit %= 10;
        }
        if increment != 0 {
            digits.insert(0, increment);
        }
        digits
    }
}
impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        let num = num as usize;
        let sum = sum as usize;

        if sum > 9 * num {
            return String::new();
        }

        let nines = sum / 9;
        let rem = sum % 9;

        let mut ans = String::with_capacity(num);

        for _ in 0..nines {
            ans.push('9');
        }

        if rem > 0 {
            ans.push(std::char::from_digit(rem as u32, 10).unwrap());
        }

        while ans.len() < num {
            ans.push('0');
        }

        ans
    }
}
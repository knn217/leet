impl Solution {
    fn digit(val: i32, idx: u32) -> i32{
        const base: i32 = 10;
        let mut digit = val;
        // Remove lower digits by division flooring
        for i in 0..(idx) {
            digit /= base;
        }
        // println!("digit after removed lower {}", digit);
        // remove upper digits: get remainder
        digit %= base;
        // println!("digit after removed upper {}", digit);
        // println!("idx {} has digit {}", idx, digit);
        digit
    }
    pub fn is_palindrome(x: i32) -> bool {
        // Negative numbers are never Palindrome
        if x < 0 { return false; }
        // Handle normal numbers
        let mut range = x.clone();
        let mut max_dec = 0;
        const base: i32 = 10;
        while (0 < range) {
            range /= 10;
            max_dec += 1;
        }
        // println!("{}", max_dec);
        for dec in 0..(max_dec/2) {
            let val_rev = Self::digit(x, max_dec - dec - 1);
            let val = Self::digit(x, dec);
            // println!("{}, {}", val, val_rev);
            // Return false if 1 pair violates the palindrome rule
            if val != val_rev { return false; }
        }
        return true;
    }
}
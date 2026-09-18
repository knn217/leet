impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Negative numbers are never Palindrome
        if x < 0 { return false; }
        // Handle normal numbers
        let mut x = x;
        let mut digits: Vec::<i32> = Vec::new();
        loop {
            let mut remainder;
            (x, remainder) = (x / 10, x % 10);
            println!("{}", remainder);
            // Push the remainder
            digits.push(remainder);
            if x == 0 { break; }
        }
        for idx in 0..digits.len() {
            let idx_rev = digits.len() - 1 - idx;
            // If reached/exceeded the middle digit, break loop
            if idx >= idx_rev { break; }
            let val = digits[idx];
            let val_rev = digits[idx_rev];
            // Return false if 1 pair violates the palindrome rule
            if val != val_rev { return false; }
        }
        return true;
    }
}
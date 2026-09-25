impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // Edge case: overflow when dividing i32::MIN by -1
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        // Determine if the result will be negative
        let is_negative = (dividend < 0) ^ (divisor < 0);

        // Convert both numbers to negative to avoid i32::MIN overflow
        let neg_dividend = if dividend < 0 { dividend } else { -dividend };
        let neg_divisor = if divisor < 0 { divisor } else { -divisor };

        let mut remaining = neg_dividend;
        let mut quotient = 0;

        // Since both numbers are negative, neg_dividend <= neg_divisor means
        // absolute value of dividend >= absolute value of divisor
        while remaining <= neg_divisor {
            let mut temp_divisor = neg_divisor;
            let mut multiple = 1;

            // Double temp_divisor using bit shifts.
            // Check overflow before shifting left to prevent panic.
            while temp_divisor >= (i32::MIN >> 1) && remaining <= (temp_divisor << 1) {
                temp_divisor <<= 1;
                multiple <<= 1;
            }

            remaining -= temp_divisor;
            quotient += multiple;
        }

        if is_negative {
            -quotient
        } else {
            quotient
        }
    }
}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        // 1. Array index matching the digit character byte value
        let phone_map: [&str; 10] = [
            "",     // 0
            "",     // 1
            "abc",  // 2
            "def",  // 3
            "ghi",  // 4
            "jkl",  // 5
            "mno",  // 6
            "pqrs", // 7
            "tuv",  // 8
            "wxyz", // 9
        ];

        let digits_bytes = digits.as_bytes();
        let mut result = Vec::new();
        let mut current_path = String::new();

        Self::backtrack(
            0,
            digits_bytes,
            &phone_map,
            &mut current_path,
            &mut result,
        );

        result
    }

    fn backtrack(
        index: usize,
        digits: &[u8],
        phone_map: &[&str; 10],
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        // Base case: processed all digits
        if index == digits.len() {
            result.push(current.clone());
            return;
        }

        // Convert ASCII byte digit (e.g., b'2') to array index (e.g., 2)
        let digit_idx = (digits[index] - b'0') as usize;
        let letters = phone_map[digit_idx];

        // Loop over characters for the digit (handles both 3 and 4 letter digits seamlessly)
        for ch in letters.chars() {
            current.push(ch);                                   // Choose
            Self::backtrack(index + 1, digits, phone_map, current, result); // Explore
            current.pop();                                      // Un-choose (Backtrack)
        }
    }
}
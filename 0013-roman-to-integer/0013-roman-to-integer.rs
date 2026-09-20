impl Solution {
    fn map(ch: &char) -> i32 {
        match *ch
        {
            'I' => {return 1;}
            'V' => {return 5;}
            'X' => {return 10;}
            'L' => {return 50;}
            'C' => {return 100;}
            'D' => {return 500;}
            'M' => {return 1000;}
            _ => {return 0;}
        }
        return 0;
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut curr_val = 0;
        for (idx, ch) in s.chars().rev().enumerate() {
            let prev_val = curr_val;
            curr_val = Self::map(&ch);
            if curr_val >= prev_val {
                sum += curr_val;
            } else {
                sum -= curr_val;
            }
        }
        sum
    }
}
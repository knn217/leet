impl Solution {
    fn get_closing(opening: &u8) -> u8 {
        match *opening {
            b'(' => { return ')' as u8; }
            b'[' => { return ']' as u8; }
            b'{' => { return '}' as u8; }
            _ => { return 0; }
        }
    }
    fn is_opening(closing: &u8) -> bool {
        match *closing {
            b'(' => { return true; }
            b'[' => { return true; }
            b'{' => { return true; }
            _ => { return false; }
        }
    }
    pub fn is_valid(s: String) -> bool {
        //  can use len for byte, since all parentheses are in u8 range
        let len = s.len();
        if len % 2 == 1 { return false; }
        let mut openings: Vec<u8> = Vec::with_capacity(len / 2 + 1);
        for idx in 0..len {
            // Handle edge cases
            if openings.len() > (len - idx) {
                // println!("Not enough elements left to resolve {} openings", openings.len());
                return false;
            }
            if openings.len() > (len / 2) {
                // println!("Number of openings {} is over half of the string", openings.len());
                return false;
            }
            // Normal cases
            let ch = s.as_bytes()[idx];
            if Self::is_opening(&ch) {
                openings.push(ch);
            } else {
                if let Some(last) = openings.pop() {
                    // println!("Completed parentheses: {}{}", last as char, ch as char);
                    if Self::get_closing(&last) != ch {
                        return false;
                    }
                } else { return false; }
            }
        }
        if !openings.is_empty() {
            // println!("There are still {} opening(s) remaining", openings.len());
            return false;
        }
        return true;
    }
}
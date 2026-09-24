impl Solution {
    fn recurs_parenthesis(token: i32, max: i32, len: usize) -> Vec<String> {
        if len == 1 { return Vec::from([String::from(")")]); }
        // if token = 0 -> must open
        if token == 0 {
            let current = String::from("(");
            let mut next_strings = Self::recurs_parenthesis(token + 1, max, len - 1);
            // println!("next_strings: {:?}", next_strings);
            for s in next_strings.iter_mut() {
                s.insert_str(0, &current);
            }
            // println!("next_strings: {:?}", next_strings);
            return next_strings;
        }
        // if token = max -> must close, reduce max token
        else if token == max {
            let current = String::from(")");
            let mut next_strings = Self::recurs_parenthesis(token - 1, max - 1, len - 1);
            for s in next_strings.iter_mut() {
                s.insert_str(0, &current);
            }
            // println!("next_strings: {:?}", next_strings);
            return next_strings;
        }
        // else: branch
        else {
            let mut vec: Vec<String> = vec![];
            {
                let current = String::from("(");
                let mut next_strings = Self::recurs_parenthesis(token + 1, max, len - 1);
                for s in next_strings.iter_mut() {
                    s.insert_str(0, &current);
                }
                vec.append(&mut next_strings);
            }
            {
                // Close parentesis, remember to reduce max token
                let current = String::from(")");
                let mut next_strings = Self::recurs_parenthesis(token - 1, max - 1, len - 1);
                for s in next_strings.iter_mut() {
                    s.insert_str(0, &current);
                }
                vec.append(&mut next_strings);
            }
            // println!("vec: {:?}", vec);
            return vec;
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::recurs_parenthesis(0, n, (n*2) as usize)
    }
}
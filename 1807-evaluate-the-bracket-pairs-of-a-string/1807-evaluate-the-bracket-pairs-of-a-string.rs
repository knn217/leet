use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        // Build the lookup table beforehand in O(K)
        let map: HashMap<&str, &str> = knowledge
            .iter()
            .map(|kv| (kv[0].as_str(), kv[1].as_str()))
            .collect();

        let mut res = String::with_capacity(s.len());
        let mut key_buf = String::new();
        let mut inside_brackets = false;

        for ch in s.chars() {
            match ch {
                '(' => {
                    inside_brackets = true;
                }
                ')' => {
                    inside_brackets = false;
                    // Hash map lookup takes O(1) average time
                    if let Some(&val) = map.get(key_buf.as_str()) {
                        res.push_str(val);
                    } else {
                        res.push('?');
                    }
                    key_buf.clear();
                }
                _ => {
                    if inside_brackets {
                        key_buf.push(ch);
                    } else {
                        res.push(ch);
                    }
                }
            }
        }

        res
    }
}
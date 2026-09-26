impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        const OFFSET: u8 = b'0';
        let mut inc = 0;
        let mut c = String::with_capacity(a.len().max(b.len()) + 1);
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let max_len = a_bytes.len().max(b_bytes.len());
        // Pad both iterators to the length of the longer string
        let a_iter = a_bytes.iter().rev().chain(std::iter::repeat(&OFFSET));
        let b_iter = b_bytes.iter().rev().chain(std::iter::repeat(&OFFSET));
        for (&val_a, &val_b) in a_iter.zip(b_iter).take(max_len) {
            // println!("{} and {}", val_a, val_b);
            let val_a = val_a - OFFSET;
            let val_b = val_b - OFFSET;
            let mut val_c = val_a + val_b + inc;
            inc = val_c / 2;
            val_c %= 2;
            c.insert(0, (val_c + OFFSET) as char);
            // println!("c: {}", c);
        }
        if inc > 0 {
            c.insert(0, (inc + OFFSET) as char);
        }
        // println!("c: {}", c);
        c
    }
}
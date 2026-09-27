impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut vec: Vec<i32> = Vec::with_capacity(num_rows);
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(num_rows);
        for row in 0..num_rows {
            vec.push(1);
            let vec_len = vec.len();
            let mut prev = 0;
            // println!("vec before: {:?}", vec);
            for (idx, num) in vec.iter_mut().enumerate() {
                // println!("prev: {}, num: {}", prev, num);
                if (idx == 0 || idx == (vec_len-1)) {
                    prev = 1;
                    continue;
                }
                *num += prev;
                prev = *num - prev;
            }
            // println!("vec after: {:?}", vec);
            res.push(vec.clone());
        }
        res
    }
}
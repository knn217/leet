impl Solution {
    fn digit(num: i32, idx: usize) -> i32 {
        const BASE: i32 = 10;
        let mut num = num;
        for _ in 0..idx {
            num /= BASE;
        }
        num % BASE
    }
    pub fn reverse(x: i32) -> i32 {
        const BASE: i32 = 10;
        //  edge cases
        if x / BASE == 0 {
            // println!("Only 1 digit");
            return x;
        }
        if x == i32::MIN {
            // println!("Value is min of i32");
            return 0;
        }
        let mut log_10: usize = 0;
        let mut x_1 = x.abs().clone();
        let x_2 = x_1.abs();
        let sign: i32 = x / x_2;
        // println!("x_1: {}", x_1);
        while x_1 > 0 {
            x_1 /= BASE;
            log_10 += 1;
        }
        // println!("log_10: {}", log_10);
        let mut cumulative_add: i32 = 0;
        for i in (0..(log_10/2)).rev() {
            let left_idx = log_10 - 1 - i;
            let right_idx = i;
            let left = Self::digit(x_2, left_idx);
            let right = Self::digit(x_2, right_idx);
            // println!("left:[{}, {}], right: [{}, {}]", left_idx, left, right_idx, right);
            // ===================
            let left_pow = 10i32.checked_pow(left_idx as u32);
            if left_pow == None {
                // println!("left_pow == None");
                return 0;
            }
            let right_pow = 10i32.checked_pow(right_idx as u32);
            if right_pow == None {
                // println!("right_pow == None");
                return 0;
            }
            let left_sub = left_pow.unwrap().checked_sub(right_pow.unwrap());
            if left_sub == None {
                // println!("left_sub == None");
                return 0;
            }
            let right_sub = right_pow.unwrap().checked_sub(left_pow.unwrap());
            if right_sub == None {
                // println!("right_sub == None");
                return 0;
            }
            let left_mul = left.checked_mul(right_sub.unwrap());
            if left_mul == None {
                // println!("left_mul == None");
                return 0;
            }
            let right_mul = right.checked_mul(left_sub.unwrap());
            if right_mul == None {
                // println!("right_mul == None");
                return 0;
            }
            let add_1 = left_mul.unwrap().checked_add(right_mul.unwrap());
            if add_1 == None {
                // println!("add_1 == None");
                return 0;
            }
            let add_2 = cumulative_add.checked_add(add_1.unwrap());
            if add_2 == None {
                // println!("add_2 == None");
                return 0;
            }
            cumulative_add = add_2.unwrap();
            // ===================
            // cumulative_add -= left * 10i32.pow(left_idx as u32);
            // cumulative_add -= right * 10i32.pow(right_idx as u32);
            // cumulative_add += left * 10i32.pow(right_idx as u32);
            // cumulative_add += right * 10i32.pow(left_idx as u32);
        }
        let res = x_2.checked_add(cumulative_add);
        if res == None {
            // println!("res == None");
            return 0;
        }
        (res.unwrap()) * sign
    }
}
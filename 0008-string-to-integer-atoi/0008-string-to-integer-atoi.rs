impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s_len = s.len();
        if s_len > 200 || s_len < 0 {
            println!("Invalid");
            return 0;
        }
        const MAX_NUM: u8 = '9' as u8;
        const MIN_NUM: u8 = '0' as u8;
        const SPACE: u8 = ' ' as u8;
        const NEGATIVE: u8 = '-' as u8;
        const POSITIVE: u8 = '+' as u8;
        const BASE: i32 = 10;
        let mut begin: bool = false;
        let mut res: i32 = 0;
        let mut sign = 1;
        for (idx, byte) in s.as_bytes().iter().enumerate(){
            if !begin && *byte == SPACE { continue; }
            if !begin && *byte == NEGATIVE {
                begin = true;
                sign = -1;
                continue;
            }
            if !begin && *byte == POSITIVE {
                begin = true;
                sign = 1;
                continue;
            }
            if !(MIN_NUM..=MAX_NUM).contains(byte) { break; }
            if !begin { begin = true; }
            // Convert byte to numeric value
            let digit = (*byte - MIN_NUM) as i32;
            // let mul = res.checked_mul(BASE);
            let (mul, overflowed_1) = res.overflowing_mul(BASE);
            if overflowed_1 {
                if sign > 0 { return i32::MAX; }
                else { return i32::MIN; }
            }
            let (add, overflowed_2) = mul.overflowing_add(digit * sign);
            if overflowed_2 {
                if sign > 0 { return i32::MAX; }
                else { return i32::MIN; }
            }
            res = add;
            // println!("{}", res);
        }
        res
    }
}
impl Solution {
    fn shift_sortable(slice: &[i32]) -> (bool, i32, i32) {
        let first = *slice.first().unwrap();
        let last = *slice.last().unwrap();
        let mut res_first = first;
        let mut res_last = last;
        if slice.len() <= 1 {return (true, res_first, res_last);}
        let mut must_rotate = false;
        for (idx, &num) in slice.iter().enumerate() {
            if idx == 0 { continue; }
            if must_rotate && (num > first) { return (false, res_first, res_last); }
            if must_rotate && (num < slice[idx-1]) { return (false, res_first, res_last); }
            if num < slice[idx-1] {
                must_rotate = true;
                res_first = num;
                res_last = slice[idx-1];
                if num > first { return (false, res_first, res_last); }
            }
        }
        (true, res_first, res_last)
    }
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len() as i32;
        let mut divisors: Vec<i32> = (1..=nums_len).filter(|i| {nums_len % i == 0}).collect();
        // println!("divisors: {:?}", divisors);
        let mut sum = 0;
        'outer: for div in divisors {
            let mut start = 0;
            let mut last_old = 0;
            while start < nums.len() {
                let slice = &nums[start..(start+div as usize)];
                // println!("slice: {:?}", slice);
                let (sortable, first, last) = Self::shift_sortable(slice);
                if !sortable { continue 'outer; }
                if first < last_old { continue 'outer; }
                last_old = last;
                start += div as usize;
            }
            sum += div;
        }
        sum
    }
}
impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        // Find all divisors of n
        let mut divs = Vec::new();
        let mut i = 1;
        while i * i <= n {
            if n % i == 0 {
                divs.push(i);
                if i * i != n {
                    divs.push(n / i);
                }
            }
            i += 1;
        }
        divs.sort();

        let mut total_sum = 0;

        for &k in &divs {
            let mut ok = true;
            for j in 0..(n / k) {
                let start = j * k;
                let end = start + k;
                let a = &nums[start..end];
                let b = &sorted_nums[start..end];

                // Check if b is a cyclic rotation of a
                let mut double_a = Vec::with_capacity(2 * k);
                double_a.extend_from_slice(a);
                double_a.extend_from_slice(a);

                if !double_a.windows(k).any(|w| w == b) {
                    ok = false;
                    break;
                }
            }
            if ok {
                total_sum += k as i32;
            }
        }

        total_sum
    }
}
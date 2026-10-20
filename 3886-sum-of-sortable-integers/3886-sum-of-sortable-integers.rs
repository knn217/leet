impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // Fast path: if nums is already sorted, all divisors are valid.
        let mut is_sorted = true;
        for i in 1..n {
            if nums[i - 1] > nums[i] {
                is_sorted = false;
                break;
            }
        }
        if is_sorted {
            let mut sum = 0;
            let mut i = 1;
            while i * i <= n {
                if n % i == 0 {
                    sum += i as i32;
                    if i * i != n {
                        sum += (n / i) as i32;
                    }
                }
                i += 1;
            }
            return sum;
        }

        // Collect and sort divisors
        let mut divisors = Vec::with_capacity(32);
        let mut i = 1;
        while i * i <= n {
            if n % i == 0 {
                divisors.push(i);
                if i * i != n {
                    divisors.push(n / i);
                }
            }
            i += 1;
        }
        divisors.sort_unstable();

        let mut total_sum = 0;
        let ptr = nums.as_ptr();

        'outer: for &k in &divisors {
            let mut prev_max = i32::MIN;
            let num_chunks = n / k;

            for j in 0..num_chunks {
                let start = j * k;
                let mut drops = 0;
                let mut min_val = i32::MAX;
                let mut max_val = i32::MIN;

                unsafe {
                    for idx in 0..k {
                        let curr = *ptr.add(start + idx);
                        let next = *ptr.add(start + (idx + 1) % k);

                        if curr > next {
                            drops += 1;
                            if drops > 1 {
                                continue 'outer;
                            }
                        }

                        if curr < min_val { min_val = curr; }
                        if curr > max_val { max_val = curr; }
                    }
                }

                if min_val < prev_max {
                    continue 'outer;
                }

                prev_max = max_val;
            }

            total_sum += k as i32;
        }

        total_sum
    }
}
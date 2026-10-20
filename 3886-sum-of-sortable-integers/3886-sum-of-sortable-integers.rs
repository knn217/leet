impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total_sum = 0;

        // Collect divisors of n
        let mut divisors = Vec::with_capacity(128);
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

        'outer: for k in divisors {
            let mut prev_max = i32::MIN;

            for chunk in nums.chunks_exact(k) {
                // Determine if 'chunk' can be cyclically rotated into a sorted subarray
                // and check if its elements are all >= prev_max.
                let mut drops = 0;
                let mut min_val = chunk[0];
                let mut max_val = chunk[0];

                for i in 0..k {
                    let curr = chunk[i];
                    let next = chunk[(i + 1) % k];

                    if curr > next {
                        drops += 1;
                        if drops > 1 {
                            continue 'outer;
                        }
                    }

                    if curr < min_val { min_val = curr; }
                    if curr > max_val { max_val = curr; }
                }

                // Check global sorted order across blocks
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
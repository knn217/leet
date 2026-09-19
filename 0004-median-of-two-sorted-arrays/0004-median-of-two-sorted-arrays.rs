impl Solution {
    fn bin_search(vector: &Vec<i32>, start: usize, end: usize, val: &i32) -> usize {
        // println!("vector: {:?}, {}, {}, {}", vector, start, end, val);
        // Return the insertion idx of val in a sorted vector
        let mut low: usize = start;
        let mut high: usize = end;
        let mut mid: usize = (high + low) / 2;
        // Edge case: vec has the same value for all elements it matches the searched value
        if (vector[low] == vector[high]) && (*val == vector[high]) {
            // For this case, count as val higher than all elements
            return high+1;
        }
        // Normal case: prioritize lesser
        loop {
            match *val {
                n if (n == vector[low]) => {
                    // println!("Value {} is equal low [{}|{}]", n, low, vector[low]);
                    return low+1;
                }
                n if (n == vector[mid]) => {
                    // println!("Value {} is equal mid [{}|{}]", n, mid, vector[mid]);
                    return mid+1;
                }
                n if (n == vector[high]) => {
                    // println!("Value {} is equal high [{}|{}]", n, high, vector[high]);
                    return high+1;
                }
                n if (n < vector[low]) => {
                    // println!("Value {} is below low [{}|{}]", n, low, vector[low]);
                    return low;
                }
                n if (n > vector[high]) => {
                    // println!("Value {} is above high [{}|{}]", n, high, vector[high]);
                    return high+1;
                }
                n if (n > vector[low] && n < vector[mid]) => {
                    // println!("Value {} is between low [{}|{}] and mid [{}|{}] of vector", n, low, vector[low], mid, vector[mid]);
                    if (low + 1) == mid {
                        // println!("mid {} is right after low {}", mid, low);
                        return mid;
                    }
                    high = mid;
                    mid = (high + low) / 2;
                }
                n if (n > vector[mid] && n < vector[high]) => {
                    // println!("Value {} is between mid [{}|{}] and high [{}|{}] of vector", n, mid, vector[mid], high, vector[high]);
                    if (mid + 1) == high {
                        // println!("high {} is right after mid {}", high, mid);
                        return high;
                    }
                    low = mid;
                    mid = (high + low) / 2;
                }
                _ => {
                    // println!("Failed to find insertion point");
                    return 0;
                }
            }
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // start with nums1 ends, find the nums1 ends' insert pos in nums2
        let len_1: usize = nums1.len();
        let len_2: usize = nums2.len();
        let median_all = (len_1 + len_2 - 1) as f64 / 2.0;
        let median_all_low = median_all.floor() as usize;
        let median_all_high = median_all.ceil() as usize;
        let odd: bool = median_all_low == median_all_high;
        // println!("Medians: {}, {}, {}", median_all, median_all_low, median_all_high);
        // Handle edge cases
        if nums1.is_empty() { return (nums2[median_all_low] + nums2[median_all_high]) as f64 / 2.0 }
        if nums2.is_empty() { return (nums1[median_all_low] + nums1[median_all_high]) as f64 / 2.0 }
        // Normal cases
        let mut head: usize = 0;
        let mut median_low = 0.0;
        let mut median_high = 0.0;
        let mut nums1_s = 0;     // This is the start idx in nums1
        let mut nums2_s = 0;     // This is the start idx in nums2
        loop {
            let nums1_s_old = nums1_s;
            let nums2_s_old = nums2_s;
            let mut start1 = -10i32.pow(6) - 1; // Use the value below min from description
            let mut start2 = -10i32.pow(6) - 1; // Use the value below min from description
            // println!("Starting points: {}, {}", nums1_s_old, nums2_s_old);
            if let Some(val) = nums1.get(nums1_s_old) { start1 = *val; }
            if let Some(val) = nums2.get(nums2_s_old) { start2 = *val; }
            if start1 > start2 {
                // println!("Moving nums2_s {}", nums2_s_old);
                nums2_s = Self::bin_search(&nums2, nums2_s_old, nums2.len() - 1, &start1);
                // println!("New nums2_s {}", nums2_s);
                head += nums2_s - nums2_s_old;
                // println!("head={}, median_low={}, median_high={}", head, median_all_low, median_all_high);
                // If median point exceeded
                if head > median_all_low {
                    // println!("median point exceeded in nums2");
                    let offset = head - median_all_low;
                    let idx_low = nums2_s - offset;
                    // println!("Getting lower median from nums2 at idx {}", idx_low);
                    median_low = *nums2.get(idx_low).unwrap() as f64;
                    if odd {
                        // println!("median high is the same as median low");
                        median_high = median_low;
                        break;
                    }
                    // nums2_s moved to right at median_low, so we have not reached median_high
                    // which is at nums1 (right after the old start) since nums2 stopped here
                    let idx_high = nums1_s_old;
                    // println!("Getting higher median from nums1 at idx {}", idx_high);
                    median_high = *nums1.get(idx_high).unwrap() as f64;
                    let high_verify = Self::bin_search(&nums2, idx_low, nums2.len() - 1, &(median_high as i32));
                    if high_verify != (idx_low + 1) {
                        // println!("Previous higher median is invalid, getting from nums2 at idx {}", idx_low + 1);
                        median_high = *nums2.get(idx_low + 1).unwrap() as f64;
                    }
                    break;
                }
                // If median point still not reached but nums2_s already exceeded nums2 length, then we can pinpoint median in nums1
                if nums2_s >= nums2.len() {
                    // println!("nums2 already exceeded");
                    let idx_low = median_all_low - nums2.len();
                    // println!("Getting lower median from nums1 at idx {}", idx_low);
                    median_low = *nums1.get(idx_low).unwrap() as f64;
                    if odd {
                        // println!("median high is the same as median low");
                        median_high = median_low;
                        break;
                    }
                    let idx_high = idx_low + 1;
                    // println!("Getting higer median from nums1 at idx {}", idx_high);
                    median_high = *nums1.get(idx_high).unwrap() as f64;
                    break;
                }
            } else {
                // println!("Moving nums1_s {}", nums1_s_old);
                nums1_s = Self::bin_search(&nums1, nums1_s_old, nums1.len() - 1, &start2);
                // println!("New nums1_s {}", nums1_s);
                head += nums1_s - nums1_s_old;
                // println!("head={}, median_low={}, median_high={}", head, median_all_low, median_all_high);
                // If median point exceeded
                if head > median_all_low {
                    // println!("median point exceeded in nums1");
                    let offset = head - median_all_low;
                    let idx_low = nums1_s - offset;
                    // println!("Getting lower median from nums1 at idx {}", idx_low);
                    median_low = *nums1.get(idx_low).unwrap() as f64;
                    if odd {
                        // println!("median high is the same as median low");
                        median_high = median_low;
                        break;
                    }
                    // nums1_s moved to right at median_low, so we have not reached median_high,
                    // which is at nums2 (right after the old start) since nums1 stopped here
                    let idx_high = nums2_s_old;
                    // println!("Getting higher median from nums2 at idx {}", idx_high);
                    median_high = *nums2.get(idx_high).unwrap() as f64;
                    let high_verify = Self::bin_search(&nums1, idx_low, nums1.len() - 1, &(median_high as i32));
                    if high_verify != (idx_low + 1) {
                        // println!("Previous higher median is invalid, getting from nums1 at idx {}", idx_low + 1);
                        median_high = *nums1.get(idx_low + 1).unwrap() as f64;
                    }
                    break;
                }
                // If median point still not reached but nums1_s already exceeded nums1 length, then we can pinpoint median in nums2
                if nums1_s >= nums1.len() {
                    // println!("nums1 already exceeded");
                    let idx_low = median_all_low - nums1.len();
                    // println!("Getting lower median from nums2 at idx {}", idx_low);
                    median_low = *nums2.get(idx_low).unwrap() as f64;
                    if odd {
                        // println!("median high is the same as median low");
                        median_high = median_low;
                        break;
                    }
                    let idx_high = idx_low + 1;
                    // println!("Getting higer median from nums2 at idx {}", idx_high);
                    median_high = *nums2.get(idx_high).unwrap() as f64;
                    break;
                }
            }
        }
        // println!("Medians: {}, {}", median_low, median_high);
        (median_low + median_high) / 2.0
    }
}
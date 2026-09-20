impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index_rev_mem = nums.len();
        let mut val_counter = 0;
        'outer: for idx in 0..nums.len() {
            // If 2 ends meet, all replacements are completed
            if idx > index_rev_mem { break; }
            // If no need to replace, continue
            if nums[idx] != val { continue; }
            // Found val, current index need replacing
            // println!("Found val in {}, need replacing", idx);
            val_counter += 1;
            // println!("Searching reverse in [{}, {}]", idx + 1, index_rev_mem);
            for idx_rev in ((idx + 1)..index_rev_mem).rev() {
                // If value can't be use for replace, continue
                if nums[idx_rev] == val {
                    // println!("Found val in {} when looking for replacement", idx_rev);
                    val_counter += 1;
                    continue;
                }
                // Found value to replace, perform replacement
                // println!("Replacing {}|{} with {}|{}", idx, nums[idx], idx_rev, nums[idx_rev]);
                nums[idx] = nums[idx_rev];
                // Mark this as the new point to look up replacements next time
                index_rev_mem = idx_rev;
                // No need to look for replacement anymore
                continue 'outer;
            }
            // Replacement not found
            // println!("Replacement not found");
            break;
        }
        // println!("{}, {}", nums.len(), val_counter);
        (nums.len() - val_counter) as i32
    }
}
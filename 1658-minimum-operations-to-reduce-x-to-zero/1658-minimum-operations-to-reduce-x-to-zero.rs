use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        // Since description give: 1 <= x <= 10^9 and 1 <= nums[i] <= 10^4
        // We can use vectors as map, since the "keys" are the numbers that can sum into x
        // => All these keys are intergers >= 0
        // *** Use regular maps if this cond is not satisfied: "keys" >= 0 ***
        let mut map_start: HashMap<i32, i32> = HashMap::from([(0, 0)]);
        let mut map_end: HashMap<i32, i32> = HashMap::from([(0, 0)]);
        let mut sum = 0;
        for (idx, num) in nums.iter().enumerate() { // Max: O(X) since 1 <= nums[i] <= 10^4
            sum += num;
            if sum > x { break; }
            // Store the number of operations (idx + 1) into map at the sum value
            map_start.insert(sum, (idx + 1) as i32);
        }
        sum = 0;
        for (idx_rev, num) in nums.iter().rev().enumerate() { // Max: O(X) since 1 <= nums[i] <= 10^4
            sum += num;
            if sum > x { break; }
            // Store the number of operations (idx_rev + 1) into map at the sum value
            map_end.insert(sum, (idx_rev + 1) as i32); // Note that idx_rev will be [0,1,2..], not [n, n-1,...]
        }
        // println!("map_start: {:?}", map_start);
        // println!("map_end: {:?}", map_end);
        let mut min = nums.len() + 1;
        for (start, addend_start) in map_start.iter() { // O(X)
            let end = x - start;
            // println!("start: {}, end: {}", start, end);
            if let Some(addend_end) = map_end.get(&end) {
                if *addend_end < 0 { continue; }
                let num_operations = addend_start + addend_end;
                if num_operations <= 0 { continue; }
                min = min.min(num_operations as usize);
                // println!("min: {}", min);
            }
        }
        if min <= nums.len() { return min as i32; }
        -1
    }
}
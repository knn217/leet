use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut answer = vec![];
        let mut map = HashMap::<i32, i32>::new();
        for idx in 0..nums.len() {
            let num = nums[idx];
            let diff = target - num;
            match map.get(&diff) {
                Some(value) => {
                    println!("Found diff: {} at index {}", diff, value);
                    answer = vec![idx as i32, *value];
                }
                None => {
                    println!("Diff not found");
                    map.insert(num, idx as i32);
                }
            }
        }
        answer
    }
}
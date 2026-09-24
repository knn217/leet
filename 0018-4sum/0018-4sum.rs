impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        // println!("nums: {:?}", nums);
        let mut res: Vec<Vec<i32>> = vec![];
        for (a, &val_a) in nums.iter().enumerate() {
            // If val_a > target then cannot find any combination = target since nums is sorted
            if val_a > 0 && val_a > target { break; }
            // iter a leaves the last 3 elements
            if a > nums.len() - 4 { break; }
            // Skip repeats in a
            if a > 0 && val_a == nums[a - 1] { continue; }
            // println!("a: {}", a);
            for (b, &val_b) in nums.iter().enumerate().skip(a+1) {
                // iter b leaves the last 2 elements
                if b > nums.len() - 3 { break; }
                // Skip repeats in a
                if b > (a+1) && val_b == nums[b - 1] { continue; }
                let mut c = b + 1;
                let mut d = nums.len() - 1;
                // println!("c: {}, d: {}", c, d);
                while c < d {
                    let val_c = nums[c];
                    let val_d = nums[d];
                    let sum = val_a + val_b + val_c + val_d;
                    if sum == target {
                        // println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
                        // println!("val_a: {}, val_b: {}, val_c: {}, val_d: {}", val_a, val_b, val_c, val_d);
                        res.push(Vec::from([val_a, val_b, val_c, val_d]));
                        c += 1;
                        d -= 1;
                        // println!("c_: {}, d_: {}", c, d);
                        // if !(c < d) { break; }
                        while c < d && nums[c] == val_c { c += 1; }
                        while c < d && nums[d] == val_d { d -= 1; }
                    }
                    else if sum < target { c += 1; }
                    else if sum > target { d -= 1; }
                }
            }
        }
        res
    }
}
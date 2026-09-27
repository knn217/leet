impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx2 = (n-1);
        for num in nums1.iter_mut().rev() {
            if *num == 0 {
                *num += nums2[idx2 as usize];
                idx2 -= 1;
                if idx2 < 0 { break; }
            } else { break; }
        }
        nums1.sort();
    }
}
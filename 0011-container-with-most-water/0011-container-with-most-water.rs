impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_vol = 0;
        let mut idx_left = 0;
        let mut idx_right = height.len() - 1;
        loop {
            if idx_left == idx_right { break; }
            let h_left = height[idx_left];
            let h_right = height[idx_right];
            let h = h_left.min(h_right);
            let volume = h * (idx_right - idx_left) as i32;
            max_vol = max_vol.max(volume);
            if h_left < h_right { idx_left += 1; }
            else { idx_right -= 1; }
        }
        max_vol
    }
}
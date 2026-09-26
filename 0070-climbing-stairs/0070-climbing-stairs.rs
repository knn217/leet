impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let sqrt_5 = 5.0_f64.sqrt();
        let phi = (1.0 + sqrt_5) / 2.0;
        // Binet's formula for F_(n + 1)
        let fib = (phi.powi(n + 1) / sqrt_5).round();
        fib as i32
    }
}
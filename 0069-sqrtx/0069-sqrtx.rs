impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 { return x; }
        let x = x as f64;
        let mut high = x;
        let mut low = 0_f64;
        let mut target = (high + low) / 2_f64;
        let mut square = f64::MAX;
        loop {
            square = target * target;
            // println!("target: {:?}, square: {:?}, square-1: {:?}, x: {:?}", target, square, square-1.0, x);
            if square == x { return target.floor() as i32; }
            // let offset = target.max(1.0);
            let offset = 1.0;
            if x < square && (square - offset) < x { return target.floor() as i32; }
            else if square < x { low = target; }
            else if square > x { high = target; }
            target = (high + low) / 2_f64;
        }
        // println!("target: {:?}, square: {:?}, square-1: {:?}, x: {:?}", target, square, square-1.0, x);
        target.floor() as i32
    }
}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 { return 0; }
        let mut max_profit = 0;
        let mut current_lowest = prices[0];
        for (idx, &price) in prices.iter().enumerate() {
            if (idx > 0) && price < current_lowest { current_lowest = price; }
            if (idx > 0) { max_profit = max_profit.max(price - current_lowest); }
        }
        max_profit
    }
}
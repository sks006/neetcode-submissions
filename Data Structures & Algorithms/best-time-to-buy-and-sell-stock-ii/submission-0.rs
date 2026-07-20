impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut total_profit=0;
            let n=prices.len();
            for i in 1..n{
                if prices[i]>prices[i-1]{
                    total_profit += prices[i]-prices[i-1]
                }
                

            }
            total_profit
    }
}

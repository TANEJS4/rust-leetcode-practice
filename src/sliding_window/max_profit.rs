use std::cmp::max;
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>)
-> i32
{
    let (mut l, mut r, mut min_buy, mut max_sell, mut profit) = (0, 1, i32::MAX, 0, 0);
    let (mut min_buy_i, mut max_sell_i) = (-2 as i32, -1 as i32);
    while r < prices.len() && l < r {
        if prices[l] > prices[r] && r < prices.len() - 1  {
            // if prices[l] is more than  prices[r] and min_buy is more than max sell,
            // slide to the right
            l = r;
        }
        if  prices[r] < min_buy {
            // if prices[l] is more than  prices[r]
            // and min_buy is more than prices[r],
            // found a new minimum, set to that.

            min_buy = prices[l];
            min_buy_i = l as i32;
            max_sell = prices[r];
            max_sell_i = r as i32;
        }
        if prices[r] > max_sell {
            max_sell_i = r as i32;
            max_sell =  prices[r];
        }
        if prices[l] < min_buy {
            min_buy_i = l as i32;
            min_buy = prices[l];
        }
        if min_buy_i < max_sell_i {
            profit = max(profit, max_sell - min_buy);
        }
        r += 1;
    }
    println!("{:}", profit);
    profit
}


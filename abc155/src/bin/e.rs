use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

const INF: i32 = 1 << 30;

fn main() {
    input! {
        mut n: Chars,
    }
    n.reverse();
    n.push('0');
    let n_size = n.len();
    let mut dp = vec![[INF; 2]; n_size+1];
    dp[0][0] = 0;
    for i in 0..n_size {
        let num = n[i] as i32 - '0' as i32;
        for a in 0..10 {
            if a - num >= 0 {
                //i: not increase, i+1: not increase
                dp[i+1][0] = min(dp[i+1][0], dp[i][0] + a + (a - num));
            } else {
                //i: not increase, i+1: increase
                dp[i+1][1] = min(dp[i+1][1], dp[i][0] + a + (a - num) + 10);
            } 
            if a - num - 1 >= 0 {
                //i: increase, i+1: not increase
                dp[i+1][0] = min(dp[i+1][0], dp[i][1] + a + (a - num - 1));
            } else {
                //i: increase, i+1: increase
                dp[i+1][1] = min(dp[i+1][1], dp[i][1] + a + (a - num - 1) + 10);
            }
        }
    }
    println!("{}", min(dp[n_size][0], dp[n_size][1]));
}

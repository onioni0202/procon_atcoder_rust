// rust doesn't have bitset library. this is TLE solution

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
        b: [[i32; w]; h]
    }
    let mut dp = vec![vec![HashSet::new(); w]; h];
    dp[0][0].insert(a[0][0] - b[0][0]);
    dp[0][0].insert(b[0][0] - a[0][0]);
    for i in 0..h {
        for j in 0..w {
            let mut nxt_dp = HashSet::new();
            if i == 0 && j == 0 {
                continue;
            }
            if i != 0 {
                for &v in &dp[i - 1][j] {
                    nxt_dp.insert(v + a[i][j] - b[i][j]);
                    nxt_dp.insert(v + b[i][j] - a[i][j]);
                }
            }
            if j != 0 {
                for &v in &dp[i][j - 1] {
                    nxt_dp.insert(v + a[i][j] - b[i][j]);
                    nxt_dp.insert(v + b[i][j] - a[i][j]);
                }
            }
            dp[i][j] = nxt_dp;
        }
    }
    let mut ans = i32::max_value();
    for &v in &dp[h - 1][w - 1] {
        ans = i32::min(ans, i32::abs(v));
    }
    println!("{}", ans);
}

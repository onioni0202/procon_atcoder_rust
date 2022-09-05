use proconio::input;
use std::collections::HashSet;

const MOD: i32 = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
        xy: [(i64,i64); m]
    }
    let mut blockarea = HashSet::new();
    for (x, y) in xy {
        blockarea.insert((x, y));
    }
    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i + j + k + 1 <= n {
                    let nxt_jump = (
                        a * (i as i64) + c * (j as i64) + e * (k as i64),
                        b * (i as i64) + d * (j as i64) + f * (k as i64),
                    );
                    if !blockarea.contains(&(nxt_jump.0 + a, nxt_jump.1 + b)) {
                        dp[i + 1][j][k] += dp[i][j][k];
                        dp[i + 1][j][k] %= MOD;
                    }
                    if !blockarea.contains(&(nxt_jump.0 + c, nxt_jump.1 + d)) {
                        dp[i][j + 1][k] += dp[i][j][k];
                        dp[i][j + 1][k] %= MOD;
                    }
                    if !blockarea.contains(&(nxt_jump.0 + e, nxt_jump.1 + f)) {
                        dp[i][j][k + 1] += dp[i][j][k];
                        dp[i][j][k + 1] %= MOD;
                    }
                }
            }
        }
    }
    let mut ans = 0;

    for i in 0..n + 1 {
        for j in 0..n + 1 {
            for k in 0..n + 1 {
                if i + j + k == n {
                    ans += dp[i][j][k];
                    ans %= MOD;
                }
            }
        }
    }

    println!("{}", ans);
}

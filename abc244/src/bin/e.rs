use proconio::input;
use proconio::marker::Usize1;
const MOD: i32 = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        s:Usize1,
        t:Usize1,
        x:Usize1,
        uv:[(Usize1, Usize1); m]
    }
    let mut dp = vec![vec![vec![0, 0]; n]; k + 1];
    dp[0][s][0] = 1;
    for i in 0..k {
        for j in 0..m {
            let (u, v) = uv[j];
            for l in 0..2 {
                dp[i + 1][v][(l + if u == x { 1 } else { 0 }) % 2] += dp[i][u][l];
                dp[i + 1][v][(l + if u == x { 1 } else { 0 }) % 2] %= MOD;
                dp[i + 1][u][(l + if v == x { 1 } else { 0 }) % 2] += dp[i][v][l];
                dp[i + 1][u][(l + if v == x { 1 } else { 0 }) % 2] %= MOD;
            }
        }
    }
    println!{"{}", dp[k][t][0]};
}

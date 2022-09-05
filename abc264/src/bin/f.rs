use proconio::input;
use proconio::marker::Chars;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: [i64; h],
        c: [i64; w],
        a: [Chars; h]
    }
    let mut dp = vec![vec![vec![INF; 4]; w]; h];
    dp[0][0][0] = 0;
    dp[0][0][1] = r[0];
    dp[0][0][2] = c[0];
    dp[0][0][3] = r[0] + c[0];
    for i in 0..h {
        for j in 0..w {
            for k in 0..4 {
                let mut x = a[i][j].to_digit(10).unwrap();
                if k & 1 != 0 {
                    x ^= 1;
                }
                if k & 2 != 0 {
                    x ^= 1;
                }
                if i + 1 < h {
                    let mut y = a[i + 1][j].to_digit(10).unwrap();
                    let nk = k & 2;
                    if k & 2 != 0 {
                        y ^= 1;
                    }
                    if x != y {
                        dp[i + 1][j][nk | 1] =
                            i64::min(dp[i + 1][j][nk | 1], dp[i][j][k] + r[i + 1]);
                    } else {
                        dp[i + 1][j][nk] = i64::min(dp[i + 1][j][nk], dp[i][j][k]);
                    }
                }

                if j + 1 < w {
                    let mut y = a[i][j + 1].to_digit(10).unwrap();
                    let nk = k & 1;
                    if k & 1 != 0 {
                        y ^= 1;
                    }
                    if x != y {
                        dp[i][j + 1][nk | 2] =
                            i64::min(dp[i][j + 1][nk | 2], dp[i][j][k] + c[j + 1]);
                    } else {
                        dp[i][j + 1][nk] = i64::min(dp[i][j + 1][nk], dp[i][j][k]);
                    }
                }
            }
        }
    }

    let mut ans = INF;
    for k in 0..4 {
        ans = i64::min(ans, dp[h - 1][w - 1][k]);
    }
    println!("{}", ans);
}

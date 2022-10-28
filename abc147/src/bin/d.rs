use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut ans = 0;
    for d in 0..60 {
        let mut zero_cnt = 0;
        let mut one_cnt = 0;
        for i in 0..n {
            if ((a[i] >> d) & 1) == 0 {
                zero_cnt += 1;
            } else {
                one_cnt += 1;
            }
        }
        ans += (((zero_cnt * one_cnt) % MOD) * ((1 << d) % MOD)) % MOD;
        ans %= MOD;
    }
    println!("{}", ans);
}

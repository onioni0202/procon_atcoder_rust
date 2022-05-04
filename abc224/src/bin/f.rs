use proconio::input;
use proconio::marker::Chars;

const MOD: i64 = 998244353;

fn modinv(mut a: i64, m: i64) -> i64 {
    let mut b: i64 = m;
    let mut u: i64 = 1;
    let mut v: i64 = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u
}

fn main() {
    input! {
        mut s: Chars,
    }
    s.reverse();
    let n = s.len();
    let mut ans: i64 = 0;
    let mut c: i64 = 1;
    let inv2 = modinv(2, MOD);
    for chr in s {
        ans += c * (chr as i64 - '0' as i64);
        c = (c * 10 + 1) * inv2;
        ans %= MOD;
        c %= MOD;
    }
    for _ in 0..n - 1 {
        ans *= 2;
        ans %= MOD;
    }
    println!("{:?}", ans);
}

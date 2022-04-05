use proconio::input;
use std::cmp::max;
use std::collections::HashMap;

fn prime_factorization(mut n: i64) -> HashMap<i64, i64> {
    let mut arr = HashMap::new();
    for i in 2..n {
        if i * i > n {break;}
        while n % i == 0 {
            (*arr.entry(i).or_insert(0)) += 1;
            n /= i;
        }
    }
    if n != 1 {(*arr.entry(n).or_insert(0)) += 1;}
    arr
}

fn modpow(mut a: i64, mut n: i64, modnum: i64) -> i64 {
    let mut res = 1;
    while n > 0 {
        if (n & 1) == 1 {
            res = res * a % modnum;
        }
        a = a * a % modnum;
        n >>= 1;
    }
    res
}

fn modinv(mut a: i64, modnum: i64) -> i64 {
    let mut b = modnum;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        std::mem::swap(&mut a, &mut b);
        b -= t * a;
        std::mem::swap(&mut u, &mut v);
        v -= t * u;
    }
    u %= modnum;
    (u + modnum) % modnum
}

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let modnum = 1_000_000_007;
    let mut primes = HashMap::new();
    for i in 0..n {
        let pf = prime_factorization(a[i]);
        for (k, v) in pf {
            let v_pointer = primes.entry(k).or_insert(0);
            *v_pointer = max(v, *v_pointer);
        }
    }
    let mut lcm = 1;
    let mut ans = 0;
    for (k, v) in primes {
        lcm *= modpow(k, v, modnum);
        lcm %= modnum;
    }
    for i in 0..n {
        ans += (lcm * modinv(a[i], modnum)) % modnum;
        ans %= modnum;
    }
    println!("{}", ans);
}

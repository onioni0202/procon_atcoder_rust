use proconio::input;

fn main() {
    input! {
        n: usize,
        at: [(i64, usize); n],
        q: usize,
        x: [i64; q]
    }
    let mut s = 0;
    let mut l = -1i64 << 60;
    let mut r = 1i64 << 60;
    for i in 0..n {
        let (a, t) = at[i];
        match t {
            1 => {
                s += a;
                l += a;
                r += a;
            }
            2 => {
                l = i64::max(l, a);
                r = i64::max(r, a);
            }
            3 => {
                l = i64::min(l, a);
                r = i64::min(r, a);
            }
            _ => unreachable!(),
        }
    }
    for i in 0..q {
        let ans = i64::min(i64::max(x[i] + s, l), r);
        println!("{}", ans);
    }
}

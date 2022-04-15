use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: usize,
        s: Chars
    }
    let mut dpl = vec![0; n];
    let mut dpr = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    let mut idx = 1;
    while l < n {
        if s[l] == 'o' {
            dpl[l] = idx;
            l += c+1;
            idx += 1;
        } else {
            l += 1;
        }
    }
    idx = k as i32;
    while r < n {
        if s[n-r-1] == 'o' {
            dpr[n-r-1] = idx;
            r += c+1;
            idx -= 1;
        } else {
            r += 1;
        }
    }
    for i in 0..n {
        if dpl[i] != 0 && dpl[i] == dpr[i] {
            println!("{}", i+1);
        }
    }
}
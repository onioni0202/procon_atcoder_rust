use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        t:Chars,
    }
    let directions = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut didx = 0;
    let mut ans = (0, 0);
    for i in 0..n {
        if t[i] == 'S' {
            ans.0 += directions[didx].0;
            ans.1 += directions[didx].1;
        } else {
            didx = (didx+1)%4;
        }
    }
    println!("{} {}", ans.0, ans.1)
}

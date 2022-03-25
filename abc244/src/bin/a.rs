use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        s:Chars
    }
    println!("{}", s[n-1]);
}

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n:usize,
        m:i32,
        mut ab: [(i32, i32); n]
    }
    ab.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    let mut q = BinaryHeap::new();
    for day in 0..m+1 {
        while !ab.is_empty() {
            if ab[ab.len()-1].0 <= day {
                let (_, b) = ab.pop().unwrap();
                q.push(b);
            } else {
                break;
            }
        }
        if !q.is_empty() {
            ans += q.pop().unwrap();
        }
    }
    println!("{}", ans);
}

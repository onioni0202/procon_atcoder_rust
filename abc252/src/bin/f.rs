use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        l: i64,
        a: [i64; n]
    }
    let mut heapq = BinaryHeap::new();
    let mut sum = 0;
    for a_v in a {
        heapq.push(Reverse(a_v));
        sum += a_v;
    }
    if l - sum > 0 {
        heapq.push(Reverse(l - sum));
    }
    let mut ans = 0;
    while let (Some(Reverse(v1)), Some(Reverse(v2))) = (heapq.pop(), heapq.pop()) {
        ans += v1 + v2;
        heapq.push(Reverse(v1 + v2));
    }
    println!("{}", ans);
}

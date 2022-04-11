use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    }
    let mut seen = vec![vec![false; n]; n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        for j in i..n {
            heap.push(Reverse((a[i][j], i, j)));
        }
    }
    let mut ans: i64 = 0;
    'outer: while let Some(Reverse((d, i, j))) = heap.pop() {
        'inner: for k in 0..n {
            if seen[i][k] && seen[k][j] {
                if d == a[i][k] + a[k][j] {
                    ans -= d;
                    break 'inner;
                } else if d > a[i][k] + a[k][j] {
                    ans = -1;
                    break 'outer;
                }
            }
        }
        seen[i][j] = true;
        seen[j][i] = true;
        ans += d;
    }
    println!("{}", ans);
}

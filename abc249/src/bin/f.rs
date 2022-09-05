use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut ty: [(i32, i64); n]
    }
    let mut ans: i64 = -1 << 60;
    let mut plus = 0;
    let mut ignored = BinaryHeap::new();
    ty.reverse();
    ty.push((1, 0));
    for (t, y) in ty {
        match t {
            1 => {
                ans = i64::max(ans, y+plus);
                if k == 0 {
                    break;
                }
                k -= 1;
            }
            2 => {
                if y >= 0 {
                    plus += y;
                } else {
                    ignored.push(y);
                }
            }
            _ => unreachable!()
        }
        if ignored.len() > k {
            plus += ignored.pop().unwrap();
        }
    }
    println!("{}", ans);
}

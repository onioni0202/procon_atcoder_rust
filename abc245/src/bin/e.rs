use proconio::input;
use std::collections::{BTreeSet};

const INF: i32 = 1 << 31;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; n],
        c: [i32; m],
        d: [i32; m]
    }
    let mut pairs = vec![];
    for i in 0..n {
        pairs.push((a[i], 0, b[i]));
    }
    for i in 0..m {
        pairs.push((c[i], 1, d[i]));
    }
    pairs.sort();
    let mut choco = BTreeSet::new();
    let mut box_pointer = 0;
    for &(_, idx, y) in &pairs {
        if idx == 0 {
            choco.insert((y, box_pointer));
            box_pointer += 1;
        } else {
            if let Some(&v) = choco.range(..(y+1, INF)).last() {
                choco.remove(&v);
            }
        }
    }
    if choco.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}

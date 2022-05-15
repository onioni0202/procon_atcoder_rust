use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        q: usize,
    }
    let mut set = BTreeSet::new();
    for pointer in 0..q {
        input! {que_idx: usize}
        match que_idx {
            1 => {
                input! {x: i64}
                set.insert((x, pointer));
            }
            2 => {
                input! {x: i64, k: usize}
                if let Some(&(ans, _)) = set.range(..(x, pointer)).rev().nth(k-1) {
                    println!("{}", ans);
                } else {
                    println!("{}", -1);
                }
                

            }
            3 => {
                input! {x: i64, k: usize}
                if let Some(&(ans, _)) = set.range((x, 0)..).nth(k-1) {
                    println!("{}", ans);
                } else {
                    println!("{}", -1);
                }
            }
            _ => unreachable!()
        }
    }
}

use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        q: usize,
        tx: [(usize, i64); q]
    }
    let n = 1 << 20;
    let mut unused: BTreeSet<_> = (0..1<<20).collect();
    let mut map = BTreeMap::new();
    for (t, x) in &tx {
        let h = x % n;
        match t {
            1 => {
                if let Some(&pos) = unused.range(h..).next() {
                    map.insert(pos, x);
                    unused.remove(&pos);
                } else if let Some(&pos) = unused.iter().next() {
                    map.insert(pos, x);
                    unused.remove(&pos);
                }
            }
            2 => {
                if let Some(&pos) = map.get(&h) {
                    println!("{}", pos);
                } else {
                    println!("{}", -1);
                }
            }
            _ => {unreachable!();}
        }
    }
}

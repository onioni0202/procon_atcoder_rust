use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    let mut bt = BTreeSet::<usize>::new();
    let mut ans = vec![!0; n + 1];
    let mut pile = vec![0; n + 1];
    let mut record = vec![0; n + 1];
    for i in 0..n {
        if let Some(&under) = bt.range(p[i]..).next() {
            record[p[i]] = under;
            pile[p[i]] = pile[under] + 1;
            bt.remove(&under);
        } else {
            pile[p[i]] += 1;
        }
        bt.insert(p[i]);
        if pile[p[i]] == k {
            bt.remove(&p[i]);
            let mut now = p[i];
            for _ in 0..k {
                ans[now] = i + 1;
                now = record[now];
            }
        }
    }
    for i in 1..=n {
        if ans[i] == !0 {
            println!("{}", -1);
        } else {
            println!("{}", ans[i]);
        }
    }
}

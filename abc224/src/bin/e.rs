use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeMap;

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        rca: [(Usize1, Usize1, i32); N]
    }
    let mut bmap = BTreeMap::<i32, Vec<usize>>::new();
    for (idx, (_, _, a)) in rca.iter().enumerate() {
        bmap.entry(-*a).or_insert(vec![]).push(idx);
    }
    let mut maxrow = vec![-1; H];
    let mut maxcol = vec![-1; W];
    let mut ans = vec![0; N];
    for (_, indice) in bmap.iter() {
        for &idx in indice {
            let (r, c) = (rca[idx].0, rca[idx].1);
            ans[idx] = i32::max(maxrow[r], maxcol[c]) + 1;
        }
        for &idx in indice {
            let (r, c) = (rca[idx].0, rca[idx].1);
            maxrow[r] = i32::max(maxrow[r], ans[idx]);
            maxcol[c] = i32::max(maxcol[c], ans[idx]);
        }
    }
    for idx in 0..N {
        println!("{}", ans[idx]);
    }
}

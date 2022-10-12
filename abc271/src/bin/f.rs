use proconio::input;
use std::collections::HashMap;

fn search(
    n: usize,
    a: &Vec<Vec<i64>>,
    start: (usize, usize),
    directions: [(usize, usize); 2],
    is_included: bool,
) -> Vec<Vec<HashMap<i64, i64>>> {
    let mut record = vec![vec![HashMap::new(); n]; n];
    for bit in 0..(1 << (n - 1)) {
        let mut now = start;
        let mut xor = a[now.0][now.1];
        for i in 0..(n - 1) {
            now.0 = now.0.wrapping_add(directions[((bit >> i) & 1)].0);
            now.1 = now.1.wrapping_add(directions[((bit >> i) & 1)].1);
            xor ^= a[now.0][now.1];
            if i == n - 2 && !is_included {
                xor ^= a[now.0][now.1];
            }
        }
        *record[now.0][now.1].entry(xor).or_insert(0) += 1;
    }
    record
}

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n]
    }
    let record1 = search(n, &a, (0, 0), [(1, 0), (0, 1)], false);
    let record2 = search(n, &a, (n - 1, n - 1), [(!0, 0), (0, !0)], true);
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for (k, v) in record1[i][j].iter() {
                if let Some(o_v) = record2[i][j].get(k) {
                    ans += v * o_v;
                }
            }
        }
    }
    println!("{}", ans);
}

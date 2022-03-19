use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n : usize,
        xy : [(i32, i32); n],
        s : Chars,
    }
    let mut map: HashMap<i32, Vec<(i32, usize)>> = HashMap::new();
    for i in 0..n {
        let (x, y) = xy[i];
        (*map.entry(y).or_insert(vec![])).push((x, i));
    }
    let mut ans = false;
    for (_, mut xs) in map {
        xs.sort();
        ans |= xs.windows(2).any(|j| s[j[0].1] == 'R' && s[j[1].1] == 'L');
    }
    println!("{}", if ans { "Yes" } else { "No" });
}

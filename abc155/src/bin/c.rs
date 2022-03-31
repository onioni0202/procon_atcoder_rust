use proconio::input;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    input! {
        n: i32,
    }
    let mut map = HashMap::new();
    let mut max_v = 0;
    for _ in 0..n {
        input! {
            name: String,
        }
        let key = map.entry(name).or_insert(0);
        *key += 1;
        max_v = max(*key, max_v);
    }
    let mut ans = vec![];
    for (k, v) in &map {
        if *v == max_v {ans.push(k);}
    }
    ans.sort();
    for k in &ans {
        println!("{}", k);
    }
}

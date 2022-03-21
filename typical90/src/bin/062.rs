use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::mem::replace;

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    }
    let mut graph = vec![vec![]; n];
    let mut q = VecDeque::new();
    let mut seen = vec![false; n];

    for (i, &(a, b)) in ab.iter().enumerate() {
        graph[a].push(i);
        graph[b].push(i);
        if ab[i].0 == i || ab[i].1 == i {
            q.push_back(i);
            seen[i] = true;
        }
    }

    let mut ans = Vec::new();
    while let Some(x) = q.pop_front(){
        ans.push(x);
        for &y in &graph[x]{
            if !replace(&mut seen[y], true){
                q.push_back(y);
            }
        }
    }

    if ans.len() < n {
        println!("{}", -1);
    } else {
        for i in (0..n).rev(){
            println!("{}", ans[i]+1);
        }
    }
}

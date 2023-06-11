use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, i32); k]
    }
    let mut que = BinaryHeap::new();
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut seen = vec![false; n];
    for (p, h) in ph {
        que.push((h, p));
    }
    while let Some((h, fr)) = que.pop() {
        if seen[fr] {continue;}
        seen[fr] = true;
        if h == 0 {continue;}
        for &to in &graph[fr] {
            if !seen[to] {
                que.push((h-1, to));
            }
        }
    }
    let mut reachable_num = 0;
    let mut ans = vec![];
    for i in 0..n {
        if seen[i] {
            reachable_num += 1;
            ans.push(i+1);
        }
    }
    let result: String = ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", reachable_num);
    println!("{}", result);
}
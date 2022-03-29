use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        u: Usize1,
        v: Usize1,
        ab: [(Usize1, Usize1); n-1]
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    let taka = bfs(u, n, &graph);
    let aoki = bfs(v, n, &graph);
    let mut ans = 0;
    for i in 0..n {
        if taka[i] < aoki[i] {
            ans = max(ans, aoki[i] - 1);
        }
    }
    println!("{}", ans);
}

fn bfs(s: usize, n: usize, graph: &Vec<Vec<usize>>) -> Vec<i32> {
    let mut dist = vec![1_000_000_000; n];
    let mut q = VecDeque::new();
    q.push_back(s);
    dist[s] = 0;
    while let Some(fr) = q.pop_front() {
        for &to in &graph[fr] {
            if dist[to] > dist[fr] + 1 {
                dist[to] = dist[fr] + 1;
                q.push_back(to);
            }
        }
    }
    dist
}

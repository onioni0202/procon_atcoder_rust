use proconio::{input, fastout};
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn dfs(now: usize, edges: &mut Vec<(usize, usize)>, seen: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
    seen[now] = true;
    for &to in &graph[now] {
        if !std::mem::replace(&mut seen[to], true) {
            edges.push((now, to));
            dfs(to, edges, seen, graph);
        }
    }
}

fn bfs(st: usize, edges: &mut Vec<(usize, usize)>, seen: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
    let mut que = VecDeque::new();
    seen[st] = true;
    que.push_back(st);
    while let Some(fr) = que.pop_front() {
        for &to in &graph[fr] {
            if !std::mem::replace(&mut seen[to], true) {
                edges.push((fr, to));
                que.push_back(to);
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m]
    }
    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut edges = vec![];
    let mut seen = vec![false; n];
    dfs(0, &mut edges, &mut seen, &graph);
    for (u, v) in edges {
        println!("{} {}", u + 1, v + 1);
    }
    let mut edges = vec![];
    let mut seen = vec![false; n];
    bfs(0, &mut edges, &mut seen, &graph);
    for (u, v) in edges {
        println!("{} {}", u + 1, v + 1);
    }
}

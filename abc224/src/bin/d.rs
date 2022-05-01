use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        m:usize,
        uv: [(usize, usize); m],
        ps: [usize; 8]
    }
    let mut init_pos = vec![!0; 9];
    let mut graph = vec![vec![]; 9];
    let mut nodes = HashMap::<Vec<usize>, i32>::new();
    let mut que = VecDeque::<Vec<usize>>::new();
    let ans = vec![0, 1, 2, 3, 4, 5, 6, 7, !0];
    for (u, v) in uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    for (idx, p) in ps.iter().enumerate() {
        init_pos[p - 1] = idx;
    }
    *nodes.entry(init_pos.clone()).or_insert(0) = 0;
    que.push_back(init_pos.clone());
    while let Some(mut pos) = que.pop_front() {
        let dist = nodes[&pos];
        for (idx, p) in pos.clone().iter().enumerate() {
            if *p != !0 {
                continue;
            }
            for &to in &graph[idx] {
                pos.swap(idx, to);
                if !nodes.contains_key(&pos) {
                    *nodes.entry(pos.clone()).or_insert(0) = dist + 1;
                    que.push_back(pos.clone());
                }
                pos.swap(idx, to);
            }
        }
    }
    if nodes.contains_key(&ans) {
        println!("{}", nodes[&ans]);
    } else {
        println!("{}", -1);
    }
}

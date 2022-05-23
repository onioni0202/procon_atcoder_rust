use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: i64 = i64::max_value();

fn dijkstra(st: usize, graph: &Vec<Vec<(usize, i64, usize)>>) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![INF; n];
    let mut heapq = BinaryHeap::new();
    dist[st] = 0;
    let mut road_indice = vec![0; n];
    heapq.push(Reverse((dist[st], 0)));
    while let Some(Reverse((cost, fr))) = heapq.pop() {
        if dist[fr] < cost {
            continue;
        }
        for &(to, nextcost, idx) in &graph[fr] {
            if dist[to] > cost + nextcost {
                road_indice[to] = idx + 1;
                dist[to] = cost + nextcost;
                heapq.push(Reverse((dist[to], to)));
            }
        }
    }
    road_indice
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m]
    }
    let mut graph = vec![vec![]; n];
    for (idx, &(a, b, c)) in abc.iter().enumerate() {
        graph[a].push((b, c, idx));
        graph[b].push((a, c, idx));
    }
    let st = 0;
    let ans = dijkstra(st, &graph);
    for i in 1..n {
        println!("{}", ans[i]);
    }
}

use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: i64 = i64::max_value();

fn dijkstra(st: usize, graph: &Vec<Vec<(usize, i64)>>) -> i64 {
    let n = graph.len();
    let mut dist = vec![INF; n];
    let mut heapq = BinaryHeap::new();
    dist[st] = 0;
    heapq.push(Reverse((dist[st], st)));
    let mut ans = INF;
    while let Some(Reverse((cost, fr))) = heapq.pop() {
        if dist[fr] < cost {
            continue;
        }
        for &(to, nextcost) in &graph[fr] {
            if to == st {
                ans = i64::min(ans, cost + nextcost);
            }
            if dist[to] > cost + nextcost {
                dist[to] = cost + nextcost;
                heapq.push(Reverse((dist[to], to)));
            }
        }
    }
    ans
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m]
    }
    let mut graph = vec![vec![]; n];
    for (a, b, c) in abc {
        graph[a].push((b, c));
    }
    for st in 0..n {
        let mindist = dijkstra(st, &graph);
        if mindist == INF {
            println!("{}", -1)
        } else {
            println!("{}", mindist);
        }
    }
}
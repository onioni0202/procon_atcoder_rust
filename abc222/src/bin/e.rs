use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashMap, VecDeque};

const MOD: i64 = 998244353;

fn modpow(mut a: i64, mut n: i64, modulo: i64) -> i64 {
    let mut res: i64 = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % modulo;
        }
        a = a * a % modulo;
        n >>= 1;
    }
    res
}

fn bfs(
    start: usize,
    goal: usize,
    graph: &Vec<Vec<usize>>,
    passing_counts: &mut HashMap<(usize, usize), usize>,
) {
    let mut que = VecDeque::new();
    let mut parents = vec![None; graph.len()];
    let mut seen = vec![false; graph.len()];
    que.push_back(start);
    seen[start] = true;
    while let Some(now) = que.pop_front() {
        if now == goal {
            break;
        }
        for &to in &graph[now] {
            if !seen[to] {
                parents[to] = Some(now);
                seen[to] = true;
                que.push_back(to);
            }
        }
    }
    let mut now = goal;
    while let Some(par) = parents[now] {
        let road_key = (usize::min(par, now), usize::max(par, now));
        *passing_counts.entry(road_key).or_insert(0) += 1;
        now = par;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i32,
        a: [Usize1; m],
        uv: [(Usize1, Usize1); n-1]
    }
    let mut graph = vec![vec![]; n];
    let mut passing_counts = HashMap::<(usize, usize), usize>::new();
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    for (&a0, &a1) in a.iter().tuple_windows() {
        bfs(a0, a1, &graph, &mut passing_counts);
    }
    let mut not_passed_counts = 0;
    for &(u, v) in &uv {
        if !passing_counts.contains_key(&(usize::min(u, v), usize::max(u, v))) {
            not_passed_counts += 1;
        }
    }
    let counts_sum: usize = passing_counts.values().sum();
    if k + counts_sum as i32 >= 0 && (k + counts_sum as i32) % 2 == 0 {
        let ans = ((k + counts_sum as i32) / 2) as usize;
        let mut dp = vec![0; ans + 1];
        dp[0] += 1;
        for &cnt in passing_counts.values() {
            let bf_dp = dp.clone();
            for idx in 0..=ans {
                if (idx + cnt) <= ans {
                    dp[idx + cnt] = (dp[idx + cnt] + bf_dp[idx]) % MOD;
                }
            }
        }
        println!("{}", (dp[ans] * modpow(2, not_passed_counts, MOD)) % MOD);
    } else {
        println!("{}", 0);
    }
}

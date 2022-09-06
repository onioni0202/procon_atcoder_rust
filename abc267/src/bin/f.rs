use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn dfs(now: usize, par: usize, dist: i32, graph: &Vec<Vec<usize>>) -> (i32, usize) {
    let mut res = (dist, now);
    for &to in &graph[now] {
        if to != par {
            let res_to = dfs(to, now, dist + 1, graph);
            if res_to.0 > res.0 {
                res = res_to;
            }
        }
    }
    res
}

fn dfs2(
    now: usize,
    par: usize,
    graph: &Vec<Vec<usize>>,
    queries: &Vec<Vec<(usize, usize)>>,
    order: &mut VecDeque<usize>,
    ans: &mut Vec<usize>,
) {
    order.push_front(now);
    for &(i, k) in &queries[now] {
        if k < order.len() {
            ans[i] = order[k] + 1;
        }
    }
    for &to in &graph[now] {
        if to != par {
            dfs2(to, now, graph, queries, order, ans);
        }
    }
    order.pop_front();
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
        q: usize,
        uk: [(Usize1, usize); q]
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    let a = dfs(0, !0, 0, &graph).1;
    let b = dfs(a, !0, 0, &graph).1;
    let mut ans = vec![!0; q];
    let mut queries = vec![vec![]; n];
    for que_i in 0..q {
        let (u, k) = uk[que_i];
        queries[u].push((que_i, k));
    }
    dfs2(a, !0, &graph, &queries, &mut VecDeque::new(), &mut ans);
    dfs2(b, !0, &graph, &queries, &mut VecDeque::new(), &mut ans);
    for i in 0..q {
        if ans[i] == !0 {
            println!("{}", -1);
        } else {
            println!("{}", ans[i]);
        }
    }
}

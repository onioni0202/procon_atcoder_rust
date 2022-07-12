use proconio::input;
use proconio::marker::Usize1;

const INF: i64 = 1 << 60;

fn dfs(
    now: usize,
    d: &Vec<i64>,
    graph: &Vec<Vec<(usize, i64)>>,
    seen: &mut Vec<bool>,
    dp: &mut Vec<(i64, i64)>,
) {
    let mut children = vec![];
    seen[now] = true;
    for &(to, w) in &graph[now] {
        if !seen[to] {
            dfs(to, d, graph, seen, dp);
            children.push((w + dp[to].0 - dp[to].1, w, to));
        }
    }
    children.sort();
    children.reverse();
    let mut cnt = 1;
    for (_, w, to) in children {
        if cnt < d[now] && dp[to].0 + w > dp[to].1 {
            dp[now].0 += dp[to].0 + w;
            dp[now].1 += dp[to].0 + w;
            cnt += 1;
        } else if cnt == d[now] && dp[to].0 + w > dp[to].1 {
            dp[now].0 += dp[to].1;
            dp[now].1 += dp[to].0 + w;
            cnt += 1;
        } else {
            dp[now].0 += dp[to].1;
            dp[now].1 += dp[to].1;
        }
    }
    if d[now] == 0 {
        dp[now].0 = -INF;
    }
}

fn main() {
    input! {
        n: usize,
        d: [i64; n],
        uvw: [(Usize1, Usize1, i64); n-1]
    }
    let mut graph = vec![vec![]; n];
    for (u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    let mut seen = vec![false; n];
    let mut dp = vec![(0, 0); n];
    dfs(0, &d, &graph, &mut seen, &mut dp);
    println!("{}", dp[0].1);
}

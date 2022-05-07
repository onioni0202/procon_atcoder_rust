use proconio::input;
use proconio::marker::Usize1;

fn dfs(
    now: usize,
    seen: &mut Vec<bool>,
    ans: &mut Vec<Vec<i32>>,
    x: &Vec<i32>,
    graph: &Vec<Vec<usize>>,
) {
    seen[now] = true;
    let mut ranking = vec![x[now]];
    for &to in &graph[now] {
        if !seen[to] {
            dfs(to, seen, ans, x, graph);
            ranking.append(&mut ans[to].clone());
        }
    }
    ranking.sort();
    for _ in 0..20 {
        if let Some(v) = ranking.pop() {
            ans[now].push(v)
        } else {
            break;
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [i32; n],
        ab: [(Usize1, Usize1); n-1],
        vk: [(Usize1, Usize1); q]
    }
    let mut graph = vec![vec![]; n];
    let mut seen = vec![false; n];
    let mut ans = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    dfs(0, &mut seen, &mut ans, &x, &graph);
    //eprintln!("{:?}", ans);
    for (v, k) in vk {
        println!("{}", ans[v][k]);
    }
}

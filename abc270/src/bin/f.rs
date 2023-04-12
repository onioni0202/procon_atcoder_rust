use proconio::{input, marker::Usize1};

const INF: i64 = 1 << 60;

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.find(self.par[x]);
        self.par[x]
    }

    fn union(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.find(parent);
        child = self.find(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.siz[root]
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

fn kruskal(n: usize, mut edges: Vec<(usize, usize, i64)>) -> i64 {
    edges.sort_by(|a, b| a.2.partial_cmp(&(b.2)).unwrap());
    let mut uf = UnionFind::new(n + 2);
    let mut all_cost = 0;
    for (u, v, cost) in edges {
        if !uf.issame(u, v) {
            uf.union(u, v);
            all_cost += cost;
        }
    }
    if uf.size(0) >= n {
        return all_cost;
    }
    return INF;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [i64; n],
        y: [i64; n],
        mut edges: [(Usize1, Usize1, i64); m]
    }
    let mut x_edges = vec![];
    let mut y_edges = vec![];
    for i in 0..n {
        x_edges.push((i, n, x[i]));
        y_edges.push((i, n + 1, y[i]));
    }

    let mut ans = kruskal(n, edges.clone());

    let mut new_edges = edges.clone();
    new_edges.append(&mut x_edges.clone());
    ans = i64::min(ans, kruskal(n, new_edges));

    let mut new_edges = edges.clone();
    new_edges.append(&mut y_edges.clone());
    ans = i64::min(ans, kruskal(n, new_edges));

    let mut new_edges = edges.clone();
    new_edges.append(&mut x_edges);
    new_edges.append(&mut y_edges);
    ans = i64::min(ans, kruskal(n, new_edges));

    println!("{}", ans);
}

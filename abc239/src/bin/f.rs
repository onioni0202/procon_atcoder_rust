use proconio::input;
use proconio::marker::Usize1;
use std::collections::{BTreeSet, HashMap};

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

fn main() {
    input! {
        n: usize,
        m: usize,
        mut d: [i32; n],
        ab: [(Usize1, Usize1); m]
    }
    let sum: i32 = d.iter().sum();
    if sum as usize != 2 * (n - 1) {
        println!("{}", -1);
        return;
    }
    let mut uf = UnionFind::new(n);
    for (a, b) in ab {
        d[a] -= 1;
        d[b] -= 1;
        if d[a] < 0 || d[b] < 0 {
            println!("{}", -1);
            return;
        }
        uf.union(a, b);
    }
    let mut children = HashMap::<usize, Vec<usize>>::new();
    let mut branch_num = BTreeSet::<(i32, usize)>::new();
    for i in 0..n {
        children.entry(uf.find(i)).or_insert(vec![]).push(i);
    }
    for (k, vs) in &children {
        let mut num = 0;
        for v in vs {
            num += d[*v];
        }
        if num <= 0 {
            println!("{}", -1);
            return;
        }
        branch_num.insert((num, *k));
    }

    while branch_num.len() >= 2 {
        let &(minlen, minkey) = branch_num.iter().next().unwrap();
        branch_num.remove(&(minlen, minkey));
        let &(maxlen, maxkey) = branch_num.iter().next_back().unwrap();
        branch_num.remove(&(maxlen, maxkey));

        let mut ans = (0, 0);
        while let Some(&minidx) = children.get(&minkey).unwrap().last() {
            if d[minidx] > 0 {
                d[minidx] -= 1;
                if minlen > 1 {
                    branch_num.insert((minlen - 1, minkey));
                }
                ans.0 = minidx + 1;
                break;
            } else if d[minidx] == 0 {
                children.entry(minkey).or_insert(vec![]).pop();
            } else {
                unreachable!();
            }
        }

        while let Some(&maxidx) = children.get(&maxkey).unwrap().last() {
            if d[maxidx] > 0 {
                d[maxidx] -= 1;
                if maxlen > 1 {
                    branch_num.insert((maxlen - 1, maxkey));
                }
                ans.1 = maxidx + 1;
                break;
            } else if d[maxidx] == 0 {
                children.entry(maxkey).or_insert(vec![]).pop();
            } else {
                unreachable!();
            }
        }

        println!("{} {}", ans.0, ans.1);
    }
}

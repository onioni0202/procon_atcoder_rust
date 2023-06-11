pub struct LazySegmentTree<T, F> {
    size: usize,
    data: Vec<T>,
    lazy: Vec<T>,
    identity: T,
    operation: F,
}

impl<T: Copy + Clone + Eq + Ord, F: Fn(T, T) -> T> LazySegmentTree<T, F> {
    pub fn new(n: usize, id: T, op: F) -> LazySegmentTree<T, F> {
        let mut size = 1;
        while size < n {
            size <<= 1;
        }
        LazySegmentTree {
            size: size,
            data: vec![id; size * 2],
            lazy: vec![id; size * 2],
            identity: id,
            operation: op,
        }
    }

    pub fn update(&mut self, a: usize, b: usize, x: T) {
        self.update_sub(0, 0, self.size, a, b, x);
    }

    pub fn query(&mut self, a: usize, b: usize) -> T {
        self.query_sub(0, 0, self.size, a, b)
    }

    fn eval(&mut self, k: usize) {
        if self.lazy[k].eq(&self.identity) {
            return;
        }
        if k < self.size - 1 {
            self.lazy[k * 2 + 1] = self.lazy[k];
            self.lazy[k * 2 + 2] = self.lazy[k];
        }
        self.data[k] = self.lazy[k];
        self.lazy[k] = self.identity;
    }

    fn update_sub(&mut self, k: usize, l: usize, r: usize, a: usize, b: usize, x: T) {
        self.eval(k);
        if a <= l && r <= b {
            self.lazy[k] = x;
            self.eval(k);
        } else if a < r && l < b {
            self.update_sub(2 * k + 1, l, (l + r) / 2, a, b, x);
            self.update_sub(2 * k + 2, (l + r) / 2, r, a, b, x);
            self.data[k] = (self.operation)(self.data[2 * k + 1], self.data[2 * k + 2]);
        }
    }

    fn query_sub(&mut self, k: usize, l: usize, r: usize, a: usize, b: usize) -> T {
        self.eval(k);
        if r <= a || b <= l {
            return self.identity;
        } else if a <= l && r <= b {
            return self.data[k];
        } else {
            let vl = self.query_sub(2 * k + 1, l, (l + r) / 2, a, b);
            let vr = self.query_sub(2 * k + 2, (l + r) / 2, r, a, b);
            return (self.operation)(vl, vr);
        }
    }
}

use std::cmp::Ordering;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, key: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while high != low {
            let mid = (low + high) / 2;
            match self[mid].cmp(key) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        high
    }
    fn upper_bound(&self, key: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while high != low {
            let mid = (low + high) / 2;
            match self[mid].cmp(key) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

use proconio::input;
const INF: i32 = 2147483647;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut stx: [(i32, i32, i32); n],
        ds: [i32; q]
    }

    let mut segtree = LazySegmentTree::new(2 * n + 10, INF, |x, y| std::cmp::min(x, y));
    let mut poses = vec![];
    stx.sort_by(|a, b| a.2.partial_cmp(&(b.2)).unwrap());
    stx.reverse();
    for i in 0..n {
        let (s, t, x) = stx[i];
        stx[i] = (i32::max(s - x, 0), i32::max(t - x, 0), x);
        poses.push(stx[i].0);
        poses.push(stx[i].1);
    }
    poses.sort();

    for (s, t, x) in stx {
        let s_idx = poses.lower_bound(&s);
        let t_idx = poses.lower_bound(&t);
        segtree.update(s_idx, t_idx, x);
    }
    for d in ds {
        let d_idx = poses.upper_bound(&d) - 1;
        let ans = segtree.query(d_idx, d_idx + 1);
        if ans == INF {
            println!("{}", -1);
        } else {
            println!("{}", ans);
        }
    }
}

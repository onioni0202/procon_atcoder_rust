use proconio::input;
use proconio::marker::{Chars, Usize1};

pub struct SegmentTree<T, F> {
    size: usize,
    data: Vec<T>,
    identity: T,
    operation: F,
}

impl<T: Copy + Clone, F: Fn(T, T) -> T> SegmentTree<T, F> {
    pub fn new(n: usize, id: T, op: F) -> SegmentTree<T, F> {
        let mut size = 1;
        while size < n {
            size <<= 1;
        }
        SegmentTree {
            size: size,
            data: vec![id; size * 2],
            identity: id,
            operation: op,
        }
    }

    pub fn update(&mut self, mut i: usize, x: T) {
        i += self.size;
        self.data[i] = x;
        i >>= 1;
        while i > 0 {
            self.data[i] = (self.operation)(self.data[i * 2], self.data[i * 2 + 1]);
            i >>= 1;
        }
    }
    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        let mut a = self.identity;
        let mut b = self.identity;
        l += self.size;
        r += self.size;
        while l < r {
            if (l & 1) == 1 {
                a = (self.operation)(a, self.data[l]);
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                b = (self.operation)(self.data[r], b);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.operation)(a, b)
    }
}

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: i32,
    }
    let mut segtree = SegmentTree::new(n + 1, 0u32, |x, y| x | y);

    for i in 0..n {
        segtree.update(i, 1 << (s[i] as usize - 'a' as usize));
    }
    for _ in 0..q {
        input! {case: i32}
        match case {
            1 => {
                input! {idx:Usize1, c:char}
                segtree.update(idx, 1 << (c as usize - 'a' as usize));
                s[idx] = c;
            }
            2 => {
                input! {l:usize, r:usize};
                let ans = segtree.query(l-1, r).count_ones();
                println!("{}", ans)
            }
            _ => {unreachable!();}
        }
    }
}

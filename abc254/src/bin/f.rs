use proconio::input;
use proconio::marker::Usize1;

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
        q: usize,
        a: [i32; n],
        b: [i32; n],
        ques: [(Usize1, Usize1, Usize1, Usize1); q]
    }
    let mut seg_a = SegmentTree::new(n, 0, |x, y| num::integer::gcd::<i32>(x, y));
    let mut seg_b = SegmentTree::new(n, 0, |x, y| num::integer::gcd::<i32>(x, y));
    for i in 0..n - 1 {
        seg_a.update(i, a[i] - a[i + 1]);
        seg_b.update(i, b[i] - b[i + 1]);
    }
    for (h1, h2, w1, w2) in ques {
        println!("{}", num::integer::gcd(a[h1]+b[w1], num::integer::gcd(seg_a.query(h1, h2), seg_b.query(w1, w2))));
    }
}

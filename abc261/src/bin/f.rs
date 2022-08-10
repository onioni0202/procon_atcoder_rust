use proconio::input;
use proconio::marker::Usize1;
use std::ops::Bound::{Included, Unbounded};
use std::collections::BTreeSet;

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

pub struct BinaryIndexedTree<T> {
    size: usize,
    data: Vec<T>,
}

impl<T: Clone + std::ops::AddAssign + Default> BinaryIndexedTree<T> {
    pub fn new(n: usize) -> BinaryIndexedTree<T> {
        BinaryIndexedTree {
            size: n,
            data: vec![T::default(); n],
        }
    }

    pub fn update(&mut self, i: usize, x: T) {
        assert!(i < self.size);
        let mut idx = i + 1;
        while idx <= self.size {
            self.data[idx - 1] += x.clone();
            idx += idx & idx.wrapping_neg();
        }
    }

    pub fn query(&self, i: usize) -> T {
        let mut res = T::default();
        let mut idx = i + 1;
        while idx > 0 {
            res += self.data[idx - 1].clone();
            idx -= idx & idx.wrapping_neg();
        }
        return res;
    }
}

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        x: [Usize1; n]
    }
    let mut bit = BinaryIndexedTree::new(n);
    let mut ans = 0;
    let mut maps = vec![BTreeSet::<(usize, usize)>::new(); n];
    for i in 0..n {
        ans += i - bit.query(x[i]);
        let lower_bound = maps[c[i]].range((Included(&(x[i], !0)), Unbounded)).count();
        ans -= lower_bound;
        maps[c[i]].insert((x[i], i));
        bit.update(x[i], 1);
    }
    println!("{}", ans);
}

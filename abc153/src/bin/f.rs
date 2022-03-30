use proconio::input;
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

const INF: i64 = 1 << 62;

fn main() {
    input! {
        n: usize,
        d: i64,
        a: i64,
        mut monster: [(i64, i64); n]
    }
    monster.sort();
    let mut acc = vec![0; n + 1];
    let mut ans = 0;
    for i in 0..n {
        let (x, h) = monster[i];
        if h + acc[i] > 0 {
            let ridx = monster.upper_bound(&(x + 2 * d, INF));
            let bomb_num = (h + acc[i] - 1) / a + 1;
            ans += bomb_num;
            acc[i] -= bomb_num * a;
            acc[ridx] += bomb_num * a;
        }
        acc[i + 1] += acc[i];
    }
    println!("{}", ans);
}

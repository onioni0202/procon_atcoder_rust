use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut d: vec![usize; n],
        ab: vec![(Usize1, Usize1); n]
    }
    let mut groups = vec![-1;n],
    for (a, b) in ab {
        d[a] -= 1;
        d[b] -= 1;
        if groups[a] == -1 && groups[b] == -1 {
            groups[a]
        }
    }
    
}

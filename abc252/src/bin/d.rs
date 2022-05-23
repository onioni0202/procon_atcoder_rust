use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut ans = 0;
    let max_iter = *a.iter().max().unwrap();
    let mut counter = vec![0; max_iter + 1];
    for i in 0..n {
        counter[a[i]] += 1;
    }
    let mut accumulate = 0;
    let sum = n;
    for i in 0..max_iter + 1 {
        ans += accumulate * counter[i] * (sum - accumulate - counter[i]);
        accumulate += counter[i];
    }
    println!("{}", ans)
}

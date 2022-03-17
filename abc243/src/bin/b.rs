use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..n {
        if a[i] == b[i] {
            ans1 += 1;
        } else if b.contains(&a[i]) {
            ans2 += 1;
        }
    }
    println!("{}", ans1);
    println!("{}", ans2);
}

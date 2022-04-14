use proconio::input;

fn make_divisors(n: i64) -> Vec<i64> {
    let mut ans = vec![];
    for i in 1..=n + 1 {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            if i != 1 {
                ans.push(i);
            }
            if i != n / i {
                ans.push(n / i);
            }
        }
    }
    ans
}

fn main() {
    input! {
        n: i64
    }
    let mut ans = make_divisors(n-1).len();
    let md = make_divisors(n);
    for div in &md {
        let mut ncopy = n;
        while ncopy % div == 0 {
            ncopy /= div;
        }
        if ncopy % div == 1{
            ans += 1;
        }
    }
    println!("{}", ans);
}

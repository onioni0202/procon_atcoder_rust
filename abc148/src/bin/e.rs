use proconio::input;

fn main() {
    input! {
        mut n:i64,
    }
    match n % 2 {
        0 => {
            let mut ans: i64 = 0;
            n /= 2;
            while n != 0 {
                n /= 5;
                ans += n;
            }
            println!("{}", ans);
        }
        1 => {
            println!("{}", 0);
        }
        _ => unreachable!(),
    }
}

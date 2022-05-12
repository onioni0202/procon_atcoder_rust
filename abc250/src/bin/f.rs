use proconio::input;

fn calc_square(i: usize, j: usize, k: usize, n: usize, xy: &Vec<(i64, i64)>) -> i64 {
    let (x1, y1) = xy[i%n];
    let (x2, y2) = xy[j%n];
    let (x3, y3) = xy[k%n];
    return i64::abs((x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3));
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut total = 0;
    for i in 1..n - 1 {
        total += calc_square(0, i, i + 1, n, &xy);
    }
    let mut ans: i64 = 9223372036854775807;
    let mut s: i64 = 0;
    let mut r = 1;
    for l in 0..n {
        while s < total {
            s += 4 * calc_square(l, r, r+1, n, &xy);
            ans = i64::min(i64::abs(s - total), ans);
            r += 1;
        }
        s -= 4 * calc_square(l, l+1, r, n, &xy);
        ans = i64::min(i64::abs(s - total), ans);
    }
    println!("{}", ans);
}


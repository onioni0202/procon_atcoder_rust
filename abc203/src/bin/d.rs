use proconio::input;

fn check(mid: i32, n: usize, k: usize, a: &Vec<Vec<i32>>) -> bool {
    let mut s = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            if mid <= a[i][j] {
                s[i + 1][j + 1] += 1;
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            s[i + 1][j + 1] += s[i + 1][j];
        }
    }

    for i in 0..n {
        for j in 0..n {
            s[i + 1][j + 1] += s[i][j + 1];
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i + k > n || j + k > n {
                continue;
            }
            let acc = s[i + k][j + k] - s[i + k][j] - s[i][j + k] + s[i][j];
            if acc <= k * k / 2 {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[[i32; n]; n]
    }
    let mut l = 0;
    let mut r = 1_000_000_005;
    for _ in 0..35 {
        let mid = (l + r) / 2;
        if check(mid, n, k, &a) {
            r = mid;
        } else {
            l = mid;
        }
    }
    println!("{}", l);
}

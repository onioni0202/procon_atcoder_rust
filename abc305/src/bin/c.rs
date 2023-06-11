use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H]
    }
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '#' {
                continue;
            }
            let mut cnt = 0;
            if h+1 < H && S[h+1][w] == '#' {
                cnt += 1;
            }
            if h >= 1 && S[h-1][w] == '#' {
                cnt += 1;
            }
            if w+1 < W && S[h][w+1] == '#' {
                cnt += 1;
            }
            if w >= 1 && S[h][w-1] == '#' {
                cnt += 1;
            }
            if cnt >= 2 {
                println!("{} {}", h+1, w+1);
            }
        }
    }
}

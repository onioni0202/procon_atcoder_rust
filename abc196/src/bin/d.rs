use proconio::input;

fn dfs(i: i32, bit: i32, a: i32, b: i32, h: i32, w: i32, cnt: &mut i32) {
    if i == h * w {
        *cnt += 1;
    }
    if (bit >> i) & 1 == 0 {
        if (i / w < h - 1) && ((bit >> (i + w)) & 1 == 0) && (a > 0) {
            dfs(i + 1, bit + (1 << i) + (1 << (i + w)), a - 1, b, h, w, cnt);
        }
        if (i % w < w - 1) && ((bit >> (i + 1)) & 1 == 0) && (a > 0) {
            dfs(i + 1, bit + (1 << i) + (1 << (i + 1)), a - 1, b, h, w, cnt);
        }
        if b > 0 {
            dfs(i + 1, bit + (1 << i), a, b - 1, h, w, cnt);
        }
    } else {
        dfs(i + 1, bit, a, b, h, w, cnt);
    }
}

fn main() {
    input! {
        h: i32,
        w: i32,
        a: i32,
        b: i32
    }
    let mut cnt = 0;
    dfs(0, 0, a, b, h, w, &mut cnt);
    println!("{}", cnt);
}

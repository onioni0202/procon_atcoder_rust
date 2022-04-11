use proconio::input;

fn main() {
    input! {
        a:i32, b:i32, c:i32, d:i32, e:i32, f:i32,
    }

    let mut ans = (0, 0);
    for i in 0..32 {
        for j in 0..32 {
            for k in 0..102 {
                for l in 0..102 {
                    let suger = c * k + d * l;
                    let water = 100 * a * i + 100 * b * j;
                    if water + suger <= f && suger * 100 <= e * water {
                        if ans.1 * (suger + water) <= ans.0 * suger {
                            ans.0 = water + suger;
                            ans.1 = suger;
                        }
                    }
                }
            }
        }
    }
    println!("{} {}", ans.0, ans.1);
}

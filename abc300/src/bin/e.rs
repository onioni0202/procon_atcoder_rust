use proconio::input;
use std::collections::HashMap;

const MOD: i64 = 998244353;
const MOD5: i64 = 598946612;

fn dfs(n: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if let Some(&ret) = memo.get(&n) {
        return ret;
    }
    match n {
        0 => return 0,
        1 => return 1,
        _ => {
            let mut ret = 0;
            for dice_num in 2..7 {
                if n % dice_num == 0 {
                    ret += dfs(n / dice_num, memo) * MOD5;
                    ret %= MOD;
                }
            }
            memo.insert(n, ret);
            return ret;
        }
    }
}

fn main() {
    input! {n:i64}
    let mut memo = HashMap::new();
    println!("{}", dfs(n, &mut memo));
}

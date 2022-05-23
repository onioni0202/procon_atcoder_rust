use proconio::input;

fn binary_search_mean(mut l: f64, mut r: f64, nums: &Vec<f64>) -> f64 {
    let mut mid = 0.0;
    for _ in 0..50 {
        mid = (l + r) / 2.0;
        if check_mean(mid, nums) {
            l = mid;
        } else {
            r = mid;
        }
    }
    mid
}

fn binary_search_median(mut l: i32, mut r: i32, nums: &Vec<i32>) -> i32 {
    let mut mid = 0;
    for _ in 0..50 {
        mid = (l + r) / 2;
        if check_median(mid, nums) {
            l = mid;
        } else {
            r = mid;
        }
    }
    mid
}

fn check_mean(mid: f64, nums: &Vec<f64>) -> bool {
    let mut dp = vec![(0.0, 0.0); nums.len() + 1];
    for (idx, &num) in nums.iter().enumerate() {
        dp[idx + 1].0 = dp[idx].1;
        dp[idx + 1].1 = f64::max(dp[idx].0, dp[idx].1) + num - mid;
    }
    return f64::max(dp[nums.len()].0, dp[nums.len()].1) > 0.0;
}

fn check_median(mid: i32, nums: &Vec<i32>) -> bool {
    let mut bigger = 0;
    let mut smaller = 0;
    let mut smaller_cnt = 0;
    for &num in nums {
        if num >= mid {
            bigger += 1;
            smaller += smaller_cnt / 2;
            smaller_cnt = 0;
        } else {
            smaller_cnt += 1;
        }
    }
    smaller += smaller_cnt / 2;
    return bigger > smaller;
}

fn main() {
    input! {
        n: usize,
        mut nums: [i32; n]
    }
    let nums_float: Vec<f64> = nums.clone().iter().map(|&x| x as f64).collect();
    let mean = binary_search_mean(0.0, 2e9, &nums_float);
    let median = binary_search_median(0, 1_000_000_005, &nums);
    println!("{}", mean);
    println!("{}", median);
}

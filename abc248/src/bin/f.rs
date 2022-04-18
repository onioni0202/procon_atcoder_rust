use proconio::input;

fn main() {
    input!{
        n:usize,
        p:i32
    }
    // 0 -> not connected, 1 -> connected
    let mut dp = vec![vec![0; 2]; n+5];
    dp[0][1] = 1;
    dp[1][0] = 1;
    for _ in 0..n-1 {
        let mut tmp = vec![vec![0; 2]; n+5];
        for j in 0..n {
            //0  -> error

            //1 | -> error

            //2 ‾
            tmp[j+2][0] = (tmp[j+2][0] + dp[j][1]) % p;
            //3 _
            tmp[j+2][0] = (tmp[j+2][0] + dp[j][1]) % p;
            //4 _|
            tmp[j+1][1] = (tmp[j+1][1] + dp[j][1]) % p;
            //5 ‾|
            tmp[j+1][1] = (tmp[j+1][1] + dp[j][1]) % p;
            //6 ‾_
            tmp[j+1][0] = (tmp[j+1][0] + dp[j][0]) % p;
            tmp[j+1][1] = (tmp[j+1][1] + dp[j][1]) % p;
            //7 ‾_|
            tmp[j][1] = (tmp[j][1] + dp[j][0]) % p;
            tmp[j][1] = (tmp[j][1] + dp[j][1]) % p;
        }
        dp = tmp;
    }
    for i in 1..n-1 {
        print!("{} ", dp[i][1]);
    }
    print!("{}\n", dp[n-1][1]);
}

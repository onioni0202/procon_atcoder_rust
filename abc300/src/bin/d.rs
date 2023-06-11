use proconio::input;

const MAX_SIZE: usize = 1_000_100;

fn main() {
    input!{
        n: i64
    }
    let mut primes = vec![];
    let mut records = vec![false; MAX_SIZE];
    for i in 2..MAX_SIZE {
        if !records[i] {
            primes.push(i as i64);
            for j in (i..MAX_SIZE).step_by(i) {
                records[j] = true;
            }
        }
    }
    let mut ans: i64 = 0;
    for aidx in 0..primes.len() {
        let a = primes[aidx];
        for bidx in aidx+1..primes.len() {
            let b = primes[bidx];
            if a * a * b > n { break; }
            for cidx in bidx+1..primes.len() {
                let c = primes[cidx];
                if a * a * b * c > n { break; }
                if a * a * b * c * c > n { break; }
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}


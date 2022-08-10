use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;

fn add(
    idx: usize, 
    timing: &Vec<Vec<usize>>, 
    counter: &mut Vec<i32>, 
    numset: &mut HashSet<usize>
) {
    for &num in &timing[idx] {
        counter[num] += 1;
        numset.insert(num);
    }
}

fn remove(
    idx: usize,
    timing: &Vec<Vec<usize>>,
    counter: &mut Vec<i32>,
    numset: &mut HashSet<usize>,
) {
    for &num in &timing[idx] {
        counter[num] -= 1;
        if counter[num] == 0 {
            numset.remove(&num);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n]
    }
    let mut timing = vec![vec![]; m];
    for i in 0..n {
        let (a, b) = ab[i];
        timing[a].push(i);
        timing[b].push(i);
    }

    let mut counter = vec![0; m];
    let mut numset = HashSet::new();
    let mut ans = vec![0; m+1];
    let mut r = 0;
    for l in 0..m {
        while r <= m {
            if numset.len() == n {
                ans[r-l-1] += 1;
                ans[m-l] -= 1;
                break;
            }
            if r < m {
                add(r, &timing, &mut counter, &mut numset);
                r += 1;
            } else {
                break;
            }
        }
        remove(l, &timing, &mut counter, &mut numset);
    }
    
    for i in 0..m {
        ans[i+1] += ans[i];
    }
    ans.pop();
    let ans_string: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
    println!("{}", ans_string.join(" "));
}

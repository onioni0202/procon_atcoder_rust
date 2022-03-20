use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut x: i64,
        s: Chars,
    }
    let mut bin = format!("{:b}", x).chars().collect::<Vec<char>>();

    for i in 0..n {
        let chr = s[i];
        match chr {
            'U' => {bin.pop();},
            'L' => {bin.push('0');},
            'R' => {bin.push('1');},
            _ => {unreachable!()},
        }
    }
    let str = bin.iter().collect::<String>();
    println!("{}", i64::from_str_radix(&str, 2).unwrap());
}
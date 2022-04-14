use proconio::input;

fn generate(now: String, n: usize, l: &mut Vec<i64>) {
    if n > 10 {
        return;
    }
    l.push(now.parse().unwrap());

    let num = now.chars().last().unwrap().to_digit(10).unwrap();
    generate(now.clone() + &(num).to_string(), n+1, l);
    if num < 9 {
        generate(now.clone() + &(num+1).to_string(), n+1, l);
    }
    if num > 0 {
        generate(now.clone() + &(num-1).to_string(), n+1, l);
    }
}

fn main() {
    input! {
        k: usize,
    }
    let mut l = vec![0];
    for i in 1..10 {
        generate(i.to_string(), 1, &mut l);
    }
    l.sort();
    println!("{}", l[k]);
}

use proconio::{input, source::line::LineSource};
use std::collections::HashSet;
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n:i32,
    }
    let nums = (1..(2 * n + 2)).collect::<Vec<i32>>();
    let mut set = HashSet::<i32>::new();
    for num in &nums {
        if set.len() == 2 * (n as usize) + 1 {
            break;
        }
        if !set.contains(num) {
            println!("{}", *num);
            stdout().flush().unwrap();
            set.insert(*num);
            input! {
                from &mut source,
                aoki:i32,
            }
            set.insert(aoki);
        }
    }
}

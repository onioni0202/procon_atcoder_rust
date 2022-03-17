use proconio::input;

fn main() {
    input! {
        mut v: i32,
        a: [i32; 3],
    }
    let mut idx: usize = 0;
    while v >= 0 {
        if v - a[idx] < 0 {
            match idx {
                0 => println!("F"),
                1 => println!("M"),
                2 => println!("T"),
                _ => println!(""),
            }
        }
        v -= a[idx];
        idx = (idx + 1) % 3;
    }
}

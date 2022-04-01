use proconio::input;

fn main() {
    input! {
        s: i32,
        mut t: i32,
        x: i32,
    }
    if s < t {
        if s <= x && x < t {
            println!("Yes")
        } else {
            println!("No")
        }
    } else {
        if s <= x && x < 24 || 0 <= x && x < t {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}

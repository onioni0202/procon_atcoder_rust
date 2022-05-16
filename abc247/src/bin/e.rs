use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        y: i32,
        mut a: [i32; n]
    }
    a.push(1 << 31);
    let mut temp = vec![];
    let mut ans: usize = 0;
    for i in 0..n+1 {
        if y > a[i] || x < a[i] {
            let mut max_place = !0;
            let mut min_place = !0;
            for i in (0..temp.len()).rev() {
                if temp[i] == x {
                    max_place = i;
                } 
                if temp[i] == y {
                    min_place = i;
                }
                let place = usize::max(max_place, min_place);
                if place != !0 {
                    ans += temp.len() - place;
                }

            }
            temp.clear();

        } else {
            temp.push(a[i]);
        }
    }
    println!("{}", ans);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [i32; (1<<n)-1]
    }
    let mut spicies = vec![];
    for (kind, &price) in c.iter().enumerate() {
        spicies.push((price as i64, kind+1));
    }
    spicies.sort();
    let mut ans: i64 = 0;
    let mut bases = vec![0; n];
    for (price, mut kind) in spicies {
        for i in 0..n {
            if ((kind >> i) & 1) == 1 {
                kind ^= bases[i];
            }
        }
        if kind == 0 {
            continue;
        }
        ans += price;
        for i in 0..n {
            if ((kind >> i) & 1) == 1 {
                bases[i] = kind;
                break;
            }
        }
        //eprintln!("{:?}", bases);
    }
    println!("{}", ans);
}

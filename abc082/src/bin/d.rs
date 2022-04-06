use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        x: i32,
        y: i32
    }
    s.push('T');
    let mut x_dists = vec![];
    let mut y_dists = vec![];
    let mut rot = 0;
    let mut dist = 0;
    for &cmd in &s {
        match cmd {
            'F' => {
                dist += 1;
            }
            'T' => {
                if dist != 0 {
                    if rot == 0 {
                        x_dists.push(dist);
                    } else {
                        y_dists.push(dist);
                    }
                }
                dist = 0;
                rot ^= 1;
            }
            _ => {
                unreachable!();
            }
        }
    }
    let mut x_can_reach = vec![false; 2 * s.len() + 10];
    let mut y_can_reach = vec![false; 2 * s.len() + 10];
    let bias = s.len() + 1;
    x_can_reach[bias] = true;
    y_can_reach[bias] = true;
    if s[0] == 'F' {
        x_can_reach[bias + x_dists.remove(0)] = true;
    }
    for x_d in x_dists {
        let mut tmp = vec![false; 2 * s.len() + 10];
        std::mem::swap(&mut tmp, &mut x_can_reach);
        for d in 0..tmp.len() {
            if tmp[d] {
                x_can_reach[d - x_d] = true;
                x_can_reach[d + x_d] = true;
            }
        }
    }
    for y_d in y_dists {
        let mut tmp = vec![false; 2 * s.len() + 10];
        std::mem::swap(&mut tmp, &mut y_can_reach);
        for d in 0..tmp.len() {
            if tmp[d] {
                y_can_reach[d - y_d] = true;
                y_can_reach[d + y_d] = true;
            }
        }
    }
    
    if x_can_reach[(x + bias as i32) as usize] && y_can_reach[(y + bias as i32) as usize] {
        println!("Yes");
    } else {
        println!("No");
    }
}

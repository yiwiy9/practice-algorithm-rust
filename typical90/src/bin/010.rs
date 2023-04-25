use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        cp:[(i32,i32); n],
        q: usize,
        lr:[(Usize1,Usize1); q],
    }

    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];

    for (i, cp_v) in cp.iter().enumerate().take(n) {
        a[i + 1] = a[i];
        b[i + 1] = b[i];

        if cp_v.0 % 2 != 0 {
            a[i + 1] += cp_v.1
        } else {
            b[i + 1] += cp_v.1
        }
    }

    for (l, r) in &lr {
        println!("{} {}", a[*r + 1] - a[*l], b[*r + 1] - b[*l])
    }
}

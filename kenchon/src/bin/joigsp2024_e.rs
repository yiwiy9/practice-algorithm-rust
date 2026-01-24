use proconio::{input, marker::Usize1};

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        ac: [(i64,Usize1); n],
    }

    let mut color_max = vec![-INF; n];
    for &(a, c) in &ac {
        color_max[c] = color_max[c].max(a);
    }

    let mut first = (-INF, n);
    let mut second = (-INF, n);
    for (i, &num) in color_max.iter().enumerate() {
        if first.0 < num {
            second = first;
            first = (num, i);
        } else if second.0 < num {
            second = (num, i)
        }
    }

    if first.0 + second.0 <= 0 {
        println!("0");
        return;
    }

    let mut ans = -first.0 - second.0;
    for &(a, c) in &ac {
        if c != first.1 {
            ans += (first.0 + a).max(0);
        } else {
            ans += (second.0 + a).max(0);
        }
    }

    println!("{}", ans);
}

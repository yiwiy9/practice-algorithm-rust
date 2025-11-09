use proconio::{input, marker::Usize1};

fn main() {
    input! {
        x: usize,
        n: usize,
        w: [usize; n],
        q: usize,
        p: [Usize1; q],
    }

    let mut cur = x;
    let mut parts = vec![false; n];
    for &p_i in &p {
        if parts[p_i] {
            parts[p_i] = false;
            cur -= w[p_i];
        } else {
            parts[p_i] = true;
            cur += w[p_i];
        }
        println!("{}", cur);
    }
}

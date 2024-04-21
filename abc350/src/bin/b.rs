use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [Usize1; q],
    }

    let mut u = vec![1; n];
    for i in t {
        u[i] ^= 1;
    }

    println!("{}", u.iter().filter(|&&x| x == 1).count());
}

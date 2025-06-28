use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    if m != n - 1 {
        println!("No");
        return;
    }

    let mut dsu = Dsu::new(n);
    let mut degrees = vec![0; n];
    for &(u, v) in &uv {
        degrees[u] += 1;
        degrees[v] += 1;
        dsu.merge(u, v);
    }

    if dsu.size(0) != n {
        println!("No");
        return;
    }

    if degrees.iter().any(|&d| d > 2) {
        println!("No");
        return;
    }

    println!("Yes");
}

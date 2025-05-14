use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut degree = vec![0; n];
    for &(a, b) in &ab {
        degree[a] += 1;
        degree[b] += 1;
        dsu.merge(a, b);
    }

    println!(
        "{}",
        if degree.iter().all(|&x| x == 2) && dsu.groups().len() == 1 {
            "Yes"
        } else {
            "No"
        }
    );
}

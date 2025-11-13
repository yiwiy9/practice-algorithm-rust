use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }

    let mut dsu = Dsu::new(n);
    for (i, &(xi, yi)) in xy.iter().enumerate() {
        for (j, &(xj, yj)) in xy.iter().enumerate() {
            if (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj) <= d * d {
                dsu.merge(i, j);
            }
        }
    }

    println!(
        "{}",
        (0..n)
            .map(|i| if dsu.same(0, i) { "Yes" } else { "No" })
            .join("\n")
    );
}

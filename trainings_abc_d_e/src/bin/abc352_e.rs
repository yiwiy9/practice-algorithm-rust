use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut cj = vec![];
    let mut a = vec![];
    for j in 0..m {
        input! {
            k: usize,
            c: usize,
            a_j: [Usize1; k],
        }
        cj.push((c, j));
        a.push(a_j);
    }

    cj.sort();

    let mut ans = 0;
    let mut dsu = Dsu::new(n);
    for &(c, j) in &cj {
        let u = a[j][0];
        for &v in &a[j] {
            if dsu.same(u, v) {
                continue;
            }
            dsu.merge(u, v);
            ans += c;
        }
    }

    println!("{}", if dsu.size(0) == n { ans as i64 } else { -1 });
}

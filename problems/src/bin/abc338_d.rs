use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
    }

    let mut imos = vec![0_i64; n + 1];
    for i in 0..m - 1 {
        let mut u = x[i];
        let mut v = x[i + 1];
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        let diff = (v - u) as i64;
        let rev_diff = n as i64 - diff;

        imos[u] += rev_diff;
        imos[v] -= rev_diff;

        imos[0] += diff;
        imos[u] -= diff;

        imos[v] += diff;
        imos[n] -= diff;
    }

    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    println!("{}", imos.iter().take(n).min().unwrap());
}

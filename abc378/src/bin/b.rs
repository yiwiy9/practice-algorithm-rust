use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        qr: [(usize,usize); n],
        m: usize,
        td: [(Usize1,usize); m],
    }

    for &(t, d) in &td {
        let (q, r) = qr[t];
        println!("{}", (d / q) * q + if d % q > r { r + q } else { r });
    }
}

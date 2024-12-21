use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a = vec![];
    for _ in 0..n {
        input! {
            l_i: usize,
            a_i: [usize; l_i],
        }
        a.push(a_i);
    }

    for _ in 0..q {
        input! {
            s: Usize1,
            t: Usize1,
        }
        println!("{}", a[s][t]);
    }
}

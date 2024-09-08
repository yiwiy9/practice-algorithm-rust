use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 0..n {
        input! {
            a_i: [Usize1; i+1],
        }
        a.push(a_i);
    }

    let mut ans = 0;
    for mut i in 0..n {
        let mut j = ans;
        if j > i {
            std::mem::swap(&mut i, &mut j);
        }
        ans = a[i][j];
    }

    println!("{}", ans + 1);
}

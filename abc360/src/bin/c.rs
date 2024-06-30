use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        w: [usize; n],
    }

    let mut box_max = vec![0; n];
    let mut box_sum = vec![0; n];
    for i in 0..n {
        box_max[a[i]] = box_max[a[i]].max(w[i]);
        box_sum[a[i]] += w[i];
    }

    let mut ans = 0;
    for i in 0..n {
        ans += box_sum[i] - box_max[i];
    }

    println!("{}", ans);
}

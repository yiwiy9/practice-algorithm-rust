use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(usize,usize,usize); n],
    }

    let mut p_sum = vec![0; n];
    for i in 0..n {
        p_sum[i] = p[i].0 + p[i].1 + p[i].2;
    }

    let p_sorted = {
        let mut p_sorted = p_sum.clone();
        p_sorted.sort_by(|a, b| b.cmp(a));
        p_sorted
    };

    let least_k = p_sorted[k - 1];

    for p_i in p_sum {
        if p_i + 300 >= least_k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

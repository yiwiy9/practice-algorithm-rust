use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [Usize1; n],
    }

    let mut idx = vec![0; n];
    for i in 0..n {
        idx[a[i]] = i;
    }

    let mut ans = vec![];
    for i in 0..n {
        let cur_idx = idx[i];
        let original_a = a[i];

        a[i] = i;
        a[cur_idx] = original_a;
        idx[i] = i;
        idx[original_a] = cur_idx;

        if i + 1 != cur_idx + 1 {
            ans.push((i + 1, cur_idx + 1));
        }
    }

    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}

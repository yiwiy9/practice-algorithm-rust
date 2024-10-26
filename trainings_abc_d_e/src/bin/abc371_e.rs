use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut num_idx = vec![vec![-1]; n + 1];
    for i in 0..n {
        num_idx[a[i]].push(i as i64);
    }
    for num in 1..=n {
        num_idx[num].push(n as i64);
    }

    // 余事象を考える
    let mut ans = 0;
    for num in 1..=n {
        ans += n * (n + 1) / 2;
        for i in 1..num_idx[num].len() {
            let l = num_idx[num][i - 1] + 1;
            let r = num_idx[num][i];
            let cnt = (r - l) as usize;
            ans -= cnt * (cnt + 1) / 2;
        }
    }

    println!("{}", ans);
}

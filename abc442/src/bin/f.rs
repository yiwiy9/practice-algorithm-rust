use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut t = vec![];
    for s_i in &s {
        let mut t_i = vec![0; n + 1];
        t_i[0] = s_i.iter().filter(|&&c| c == '.').count() as i64;
        for i in 0..n {
            t_i[i + 1] = t_i[i] + if s_i[i] == '.' { -1 } else { 1 };
        }
        t.push(t_i);
    }

    let mut dp = t[0].clone();
    for t_i in t.iter().skip(1) {
        let mut next_dp = vec![0; n + 1];
        let mut cur_min = 1 << 60;
        for (j, &num) in t_i.iter().enumerate().rev() {
            cur_min = cur_min.min(dp[j]);
            next_dp[j] = cur_min + num;
        }
        dp = next_dp;
    }

    println!("{}", dp.iter().min().unwrap());
}

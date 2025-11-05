use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    }

    let mut t_front_cnt = vec![0; t.len() + 1];
    let mut t_back_cnt = vec![0; t.len() + 1];
    for s_i in &s {
        let mut j = 0;
        for &c in s_i {
            if j < t.len() && c == t[j] {
                j += 1;
            }
        }
        t_front_cnt[j] += 1;

        let mut j = t.len();
        for &c in s_i.iter().rev() {
            if j > 0 && c == t[j - 1] {
                j -= 1;
            }
        }
        t_back_cnt[t.len() - j] += 1;
    }

    let mut t_back_cnt_cum = vec![0; t.len() + 2];
    for i in 0..t.len() + 1 {
        t_back_cnt_cum[i + 1] = t_back_cnt_cum[i] + t_back_cnt[i];
    }

    let mut ans = 0_usize;
    for (i, &cnt) in t_front_cnt.iter().enumerate() {
        ans += cnt * (t_back_cnt_cum[t.len() + 1] - t_back_cnt_cum[t.len() - i]);
    }

    println!("{}", ans);
}

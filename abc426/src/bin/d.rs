use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        // 変えない部分をどう全探索するか

        let mut s_zero = vec![0; n + 1];
        let mut s_one = vec![0; n + 1];
        for i in 0..n {
            s_zero[i + 1] = s_zero[i];
            s_one[i + 1] = s_one[i];
            if s[i] == '0' {
                s_zero[i + 1] += 1;
            } else {
                s_one[i + 1] += 1;
            }
        }

        let mut ans = INF;
        let mut left = 0;
        for right in 0..n {
            if s[left] == s[right] {
                continue;
            }

            let zero_cnt = s_zero[left] + s_zero[n] - s_zero[right];
            let one_cnt = s_one[left] + s_one[n] - s_one[right];
            if s[left] == '0' {
                ans = ans.min(zero_cnt * 2 + one_cnt);
            } else {
                ans = ans.min(zero_cnt + one_cnt * 2);
            }
            left = right;
        }

        if s[left] == '0' {
            ans = ans.min(s_zero[left] * 2 + s_one[left]);
        } else {
            ans = ans.min(s_zero[left] + s_one[left] * 2);
        }

        println!("{}", ans);
    }
}

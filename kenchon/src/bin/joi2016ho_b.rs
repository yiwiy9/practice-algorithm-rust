use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut left_j_cnt = vec![0; n];
    if s[0] == 'J' {
        left_j_cnt[0] = 1;
    }
    for i in 1..n {
        left_j_cnt[i] = left_j_cnt[i - 1] + if s[i] == 'J' { 1 } else { 0 };
    }

    let mut right_i_cnt = vec![0; n];
    if s[n - 1] == 'I' {
        right_i_cnt[n - 1] = 1;
    }
    for i in (0..n - 1).rev() {
        right_i_cnt[i] = right_i_cnt[i + 1] + if s[i] == 'I' { 1 } else { 0 };
    }

    let mut base: usize = 0;
    let mut left_j_added = 0;
    let mut right_i_added = 0;
    for i in 0..n {
        if s[i] == 'O' {
            base += left_j_cnt[i] * right_i_cnt[i];
            left_j_added += right_i_cnt[i];
            right_i_added += left_j_cnt[i];
        }
    }

    let mut ans = base + left_j_added; // 先頭に'J'追加
    ans = ans.max(base + right_i_added); // 末尾に'I'追加

    // 間に'O'追加
    for i in 0..n {
        ans = ans.max(base + right_i_cnt[i] * left_j_cnt[i]);
    }

    println!("{}", ans);
}

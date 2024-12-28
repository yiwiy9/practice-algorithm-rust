use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut k_minus_idx = 0;
    let mut k_idx_start = 0;
    let mut k_idx_end = 0;
    let mut count = if s[0] == '1' { 1 } else { 0 };
    for (i, &s_i) in s.iter().enumerate() {
        if i != 0 && s_i != s[i - 1] {
            if s_i == '1' {
                count += 1;
                if count == k {
                    k_idx_start = i;
                }
            } else if count == k - 1 {
                k_minus_idx = i;
            } else if count == k {
                k_idx_end = i;
                break;
            }
        }
    }
    if k_idx_end == 0 {
        k_idx_end = n;
    }

    println!(
        "{}{}{}{}",
        &s[..k_minus_idx].iter().collect::<String>(),
        &s[k_idx_start..k_idx_end].iter().collect::<String>(),
        &s[k_minus_idx..k_idx_start].iter().collect::<String>(),
        &s[k_idx_end..].iter().collect::<String>(),
    );
}

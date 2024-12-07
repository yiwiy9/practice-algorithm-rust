use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut compressed = vec![];
    let mut cur = s[0];
    let mut cnt = 1;
    for &c in &s[1..] {
        if c == cur {
            cnt += 1;
        } else {
            compressed.push((cur, cnt));
            cur = c;
            cnt = 1;
        }
    }
    compressed.push((cur, cnt));

    let mut one_cnt = 0;
    let mut k_minus_one_idx = n;
    let mut k_idx = n;
    for (i, &(c, _)) in compressed.iter().enumerate() {
        if c == '1' {
            one_cnt += 1;
        }
        if k_minus_one_idx == n && one_cnt == k - 1 {
            k_minus_one_idx = i;
        }
        if k_idx == n && one_cnt == k {
            k_idx = i;
            break;
        }
    }

    let mut new_compressed = vec![];
    new_compressed.extend_from_slice(&compressed[..=k_minus_one_idx]);
    new_compressed.push(compressed[k_idx]);
    new_compressed.extend_from_slice(&compressed[k_minus_one_idx + 1..k_idx]);
    new_compressed.extend_from_slice(&compressed[k_idx + 1..]);

    for (c, cnt) in new_compressed {
        for _ in 0..cnt {
            print!("{}", c);
        }
    }
    println!();
}

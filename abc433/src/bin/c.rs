use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    // 778899

    let mut compressed = vec![];
    let mut cur = s[0];
    let mut cnt = 1;
    for i in 1..s.len() {
        if s[i] == cur {
            cnt += 1;
        } else {
            compressed.push((cur, cnt));
            cur = s[i];
            cnt = 1;
        }
    }
    compressed.push((cur, cnt));

    let mut ans = 0;
    for i in 0..compressed.len() - 1 {
        let prev_num = compressed[i].0 as usize - '0' as usize;
        let next_num = compressed[i + 1].0 as usize - '0' as usize;
        if prev_num + 1 == next_num {
            ans += compressed[i].1.min(compressed[i + 1].1);
        }
    }

    println!("{}", ans);
}

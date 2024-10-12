use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    }

    let mut ans = 0;
    for bit in 0..(1 << n) {
        let mut cnt = vec![0; 26];
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                for &c in &s[i] {
                    cnt[c as usize - 'a' as usize] += 1;
                }
            }
        }
        ans = ans.max(cnt.iter().filter(|&&c| c == k).count());
    }

    println!("{}", ans);
}

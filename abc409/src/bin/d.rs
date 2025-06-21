use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        let mut l_pos = n;
        for i in 0..n - 1 {
            if s[i] as u8 - 'a' as u8 > s[i + 1] as u8 - 'a' as u8 {
                l_pos = i;
                break;
            }
        }

        if l_pos == n {
            println!("{}", s.iter().join(""));
            continue;
        }

        let mut r_pos = n;
        for i in l_pos + 2..n {
            if s[i] as u8 - 'a' as u8 > s[l_pos] as u8 - 'a' as u8 {
                r_pos = i;
                break;
            }
        }

        let mut ans = s[0..l_pos]
            .iter()
            .chain(s[l_pos + 1..r_pos].iter())
            .collect::<Vec<_>>();

        ans.push(&s[l_pos]);

        if r_pos < n {
            ans.extend(s[r_pos..].iter());
        }

        println!("{}", ans.iter().join(""));
    }
}

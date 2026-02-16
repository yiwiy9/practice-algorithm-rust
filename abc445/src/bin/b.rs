use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let m = s.iter().map(|s_i| s_i.len()).max().unwrap();

    for s_i in &s {
        let dot_cnt = (m - s_i.len()) / 2;
        let mut ans = std::iter::repeat('.').take(dot_cnt).collect_vec();
        ans.extend(s_i.iter());
        ans.extend(std::iter::repeat('.').take(dot_cnt));

        println!("{}", ans.iter().join(""));
    }
}

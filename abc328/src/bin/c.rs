use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(Usize1,Usize1); q],
    }

    let mut s_cnt = vec![0; n];
    for i in 1..n {
        s_cnt[i] = s_cnt[i - 1];
        if s[i] == s[i - 1] {
            s_cnt[i] += 1;
        }
    }

    for &(l, r) in &lr {
        println!("{}", s_cnt[r] - s_cnt[l]);
    }
}

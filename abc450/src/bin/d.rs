use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut mod_a = a.iter().map(|&a_i| a_i % k).collect_vec();
    mod_a.sort();
    mod_a.extend_from_slice(&mod_a.iter().map(|&mod_a_i| mod_a_i + k).collect_vec());

    let mut ans: usize = 1 << 60;
    for i in 0..n {
        ans = ans.min(mod_a[i + n - 1] - mod_a[i]);
    }

    println!("{}", ans);
}

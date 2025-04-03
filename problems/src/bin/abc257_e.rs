use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; 9],
    }

    let mut min_c = n + 1;
    let mut min_c_idx = 0;
    for i in 0..9 {
        if c[i] <= min_c {
            min_c = c[i];
            min_c_idx = i;
        }
    }

    let mut cnt = vec![0; 9];
    cnt[min_c_idx] = n / min_c;

    let mut cur = (n / min_c) * min_c;
    for i in (min_c_idx + 1..9).rev() {
        let mut j = 0;
        while cur + c[i] - c[min_c_idx] <= n {
            cur += c[i] - c[min_c_idx];
            j += 1;
        }
        cnt[i] = j;
        cnt[min_c_idx] -= j;
    }

    let mut ans = vec![];
    for i in (0..9).rev() {
        for _ in 0..cnt[i] {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.iter().join(""));
}

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    let mut cnt = vec![0; n + 1];
    for &x in &a {
        if x != -1 {
            cnt[x as usize] += 1;
        }
    }

    if cnt.iter().any(|&x| x > 1) {
        println!("No");
        return;
    }

    let mut cur = 1;
    for i in 0..n {
        if a[i] == -1 {
            while cnt[cur] == 1 {
                cur += 1;
            }
            a[i] = cur as i64;
            cur += 1;
        }
    }

    println!("Yes");
    println!("{}", a.iter().join(" "));
}

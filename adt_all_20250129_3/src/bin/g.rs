use proconio::input;

/// 別解: https://atcoder.jp/contests/adt_all_20250129_3/editorial/3786
/// 調和級数なので、２重ループしても間に合う
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = vec![0; 200_001];
    for &x in &a {
        cnt[x] += 1;
    }

    let mut ans = 0_usize;
    for &x in &a {
        for i in 1..=((x as f64).sqrt() as usize) {
            if x % i == 0 {
                if x / i == i {
                    ans += cnt[i] * cnt[i];
                } else {
                    ans += cnt[i] * cnt[x / i] * 2;
                }
            }
        }
    }

    println!("{}", ans);
}

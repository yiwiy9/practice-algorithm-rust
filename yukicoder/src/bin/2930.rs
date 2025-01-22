use proconio::input;
use std::collections::HashSet;

/// https://yukicoder.me/problems/no/2930
/// https://yukicoder.me/problems/no/2930/editorial
///
/// MEXの性質を考える
/// ri = mex(Ai..Ari)>=Mとなる最小の整数とする
/// そのとき、mex(Ai..Ari)<=mex(Ai..Ari+1)<=...<=mex(Ai..An)が成り立つ。
///
/// あとは、尺取り法で ri を求めて、左端: l を動かしていく
/// ri が求まるたびに、k = ri-l+1,...,n-l+1 の答えに +1 していく
/// これは、imos法で高速に求めることができる
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    if m == 0 {
        println!(
            "{}",
            (1..=n)
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );
        return;
    }

    let mut k_imos = vec![0_i64; n + 2];
    let mut cnt = vec![0; m];
    let mut set = HashSet::new();
    let mut r = 0;
    for l in 0..n {
        while r < n && set.len() < m {
            if a[r] < m {
                set.insert(a[r]);
                cnt[a[r]] += 1;
            }
            r += 1;
        }
        if set.len() < m {
            break;
        }

        k_imos[r - l] += 1;
        k_imos[n - l + 1] -= 1;

        if a[l] < m {
            cnt[a[l]] -= 1;
            if cnt[a[l]] == 0 {
                set.remove(&a[l]);
            }
        }
    }

    for i in 1..n + 1 {
        k_imos[i + 1] += k_imos[i];
    }

    println!(
        "{}",
        k_imos
            .iter()
            .skip(1)
            .take(n)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    )
}

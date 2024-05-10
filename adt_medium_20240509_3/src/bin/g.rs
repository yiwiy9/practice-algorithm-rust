use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/adt_medium_20240509_3/tasks/abc288_d
 * https://atcoder.jp/contests/adt_medium_20240509_3/editorial/5660
 *
 * 階差数列X'を考える
 * => Xi,...,Xi+k-1区間に対するcの加算が、X'i-1にcをX'i+k-1に-cを加算することに対応する
 * => 「0≤i≤n−K を満たす整数 i および、整数 c を選び、 Xi′ に c を、 Xi+K′ に −c をそれぞれ加算する」操作を通して、X'を全て0にできるかを考える
 * => 上記の操作のみを行うということは、i mod K (=r) のX'の和が常に一定であることになる
 * => したがって、すべての余り r=0,1,…,K−1 について、X'の和が0であるかを確認すればよい
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
        lr: [(Usize1,Usize1); q],
    }

    // let mut seq_a = vec![];
    // seq_a.push(a[0]);
    // for i in 0..n - 1 {
    //     seq_a.push(a[i + 1] - a[i]);
    // }
    // seq_a.push(0 - a[n - 1]);

    // let mut s = vec![vec![0; n + 1]; k];
    // for i in 0..n {
    //     for j in 0..k {
    //         s[j][i + 1] = s[j][i] + if i % k == j { seq_a[i] } else { 0 };
    //     }
    // }

    let mut s = vec![vec![0; n + 1]; k];
    for i in 0..n {
        for j in 0..k {
            s[j][i + 1] = s[j][i] + if i % k == j { a[i] } else { 0 };
        }
    }

    for (l, r) in lr {
        let mut ok = true;
        for j in 0..k {
            if s[j][r + 1] - s[j][l] != s[0][r + 1] - s[0][l] {
                ok = false;
                break;
            }
        }
        println!("{}", if ok { "Yes" } else { "No" });
    }
}

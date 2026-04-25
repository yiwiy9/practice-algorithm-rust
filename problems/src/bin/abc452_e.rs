use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;

/// https://atcoder.jp/contests/abc452/tasks/abc452_e
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   `j` を固定し、`floor(i / j)` が同じになる連続区間ごとの話にする。
/// - それについて、何が分かれば答えになる？
///   各区間の `sum A_i` と `sum A_i * i` が分かれば、`sum A_i * (i mod j)` をまとめて計算できる。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   `i mod j = i - j * floor(i / j)` で、区間内では `floor(i / j)` が一定で、全区間数も調和級数なので各 `i mod j` を個別に見なくてよい。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   `A_i` と `A_i * i` の累積和を作り、各 `j` について半開区間 `[j*q, j*(q+1))` を走査して区間寄与を足す。
///
/// 解説AC: https://atcoder.jp/contests/abc452/editorial/18408
/// 思いつき方:
/// `i` or `j` を固定したときに、もう一方の和を高速で求められるか？
/// `j` を固定して `sum A_i * (i mod j)` を高速化したい。
/// `mod` を直接扱いにくいので、`i = j * floor(i / j) + (i mod j)` に戻して、`floor(i / j)` が一定な区間を見る。
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    // prefix[x] = 1 <= i < x の範囲の和。半開区間 [l, r) は prefix[r] - prefix[l]。
    let mut prefix_a = vec![Mint::new(0); n + 2];
    let mut prefix_ai = vec![Mint::new(0); n + 2];
    for i in 1..=n {
        prefix_a[i + 1] = prefix_a[i] + Mint::new(a[i - 1]);
        prefix_ai[i + 1] = prefix_ai[i] + Mint::new(a[i - 1]) * Mint::new(i);
    }

    let mut ans = Mint::new(0);
    for j in 1..=m {
        let mut sum_mod = Mint::new(0);

        // 間隔のループ（調和級数）
        // 各 j の区間数は O(n / j + 1) なので、全体で Σ O(n/j + 1) = O(n log n + m)。
        for q in 0.. {
            let l = (j * q).max(1);
            if l > n {
                break;
            }
            let r = (j * (q + 1)).min(n + 1);

            // 区間 [l, r) では floor(i / j) = q なので、
            // A_i * (i mod j) = A_i * i - A_i * j * q をまとめて足せる。
            let section_a = prefix_a[r] - prefix_a[l];
            let section_ai = prefix_ai[r] - prefix_ai[l];
            sum_mod += section_ai - section_a * Mint::new(j * q);
        }

        ans += Mint::new(b[j - 1]) * sum_mod;
    }

    println!("{}", ans.val());
}

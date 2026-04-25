use itertools::Itertools;
use proconio::input;

/// https://atcoder.jp/contests/abc450/tasks/abc450_d
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   操作しても各要素の `mod K` は変わらないので、円環上の `N` 個の余りだけ持つ。
/// - それについて、何が分かれば答えになる？
///   `N` 個の余りをすべて含む最短の円弧長が分かれば、最小 range になる。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   切れ目を最大 gap に置けばよいので、元の値や各要素に何回 `K` を足したかは不要。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   余りを sort し、`+K` 複製した列で長さ `N` の連続区間幅 `B[i+N-1]-B[i]` の最小を取る。
///
/// AC: 17分
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut b = a.iter().map(|&a_i| a_i % k).collect_vec();
    b.sort();
    b.extend_from_slice(&b.iter().map(|&b_i| b_i + k).collect_vec());

    let mut ans: usize = 1 << 60;
    for i in 0..n {
        ans = ans.min(b[i + n - 1] - b[i]);
    }

    println!("{}", ans);
}

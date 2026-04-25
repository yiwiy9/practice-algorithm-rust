use proconio::input;

/// https://atcoder.jp/contests/abc423/tasks/abc423_e
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   クエリ `[L, R]` ごとに、各 `A_j` が何回足されるかの話にする。
/// - それについて、何が分かれば答えになる？
///   各 `j` の寄与回数 `(j-L+1) * (R-j+1)` が分かれば、`A_j * 回数` の和が答えになる。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   区間和を全部列挙しなくても、`A_j` を含む部分配列は左端と右端の選び方だけで数えられる。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   寄与式を展開し、`A_j`, `A_j * j`, `A_j * j^2` の累積和で各クエリを `O(1)` で答える。
///
/// 解説AC: https://atcoder.jp/contests/abc423/editorial/13865
/// 思いつき方:
/// `sum_{l,r} sum_j A_j` は `l,r` を固定すると重いので、和の順番を入れ替えて `A_j` 固定で考える。
/// `A_j` が含まれる条件は `L <= l <= j <= r <= R` なので、左端が `j-L+1` 通り、右端が `R-j+1` 通り。
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }

    let mut prefix_a = vec![0_i64; n + 1];
    let mut prefix_aj = vec![0_i64; n + 1];
    let mut prefix_ajj = vec![0_i64; n + 1];
    for j in 1..=n {
        let a_j = a[j - 1];
        let j_i64 = j as i64;
        prefix_a[j] = prefix_a[j - 1] + a_j;
        prefix_aj[j] = prefix_aj[j - 1] + a_j * j_i64;
        prefix_ajj[j] = prefix_ajj[j - 1] + a_j * j_i64 * j_i64;
    }

    for (l, r) in lr {
        let sum_a = prefix_a[r] - prefix_a[l - 1];
        let sum_aj = prefix_aj[r] - prefix_aj[l - 1];
        let sum_ajj = prefix_ajj[r] - prefix_ajj[l - 1];

        // A_j * (j-L+1) * (R-j+1)
        // = A_j * (-j^2 + (L+R)j - (L-1)(R+1))
        let ans = -sum_ajj + (l + r) as i64 * sum_aj - (l as i64 - 1) * (r as i64 + 1) * sum_a;
        println!("{}", ans);
    }
}

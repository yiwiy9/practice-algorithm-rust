use proconio::input;
const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc419/tasks/abc419_e
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   添字 `i % L` ごとのグループに分け、各グループを最終的にどの余り `r` に揃えるかだけ持つ。
/// - それについて、何が分かれば答えになる？
///   各グループの余り `r` への変更コストと、選んだ余りの総和が `0 mod M` になる最小コストが分かればよい。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   長さ `L` の部分列和だけが条件なので、同じ `i % L` の要素は同じ余りに揃える必要があり、元の値そのものは不要。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   `cost[pos][r]` を前計算し、グループごとに `dp[sum_mod]` を更新して最終的な `dp[0]` を答える。
///
/// AC: 25分（言語化は端折った）
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
    }

    let mut b = vec![vec![0; m]; l];
    for (i, &a_i) in a.iter().enumerate() {
        let l_i = i % l;
        for j in 0..m {
            let cnt = if j >= a_i { j - a_i } else { m + j - a_i };
            b[l_i][j] += cnt;
        }
    }

    let mut dp = b[0].clone();
    for l_i in 1..l {
        let mut next_dp = vec![INF; m];
        for j in 0..m {
            for b_j in 0..m {
                let next_j = (j + b_j) % m;
                next_dp[next_j] = next_dp[next_j].min(dp[j] + b[l_i][b_j]);
            }
        }
        dp = next_dp;
    }

    println!("{}", dp[0]);
}

use proconio::input;

/**
 * ビットDP(bit DP)
 * https://atcoder.jp/contests/abc318/tasks/abc318_d
 * https://atcoder.jp/contests/abc318/editorial/7066
 *
 * dp[b] := bのうち立っているビットに関してその頂点を選ぶか選ばないかを決定した際の、選んだ辺の重みの最大値
 * => bの昇順に求めていく (0番目の頂点から確認を済ませていく)
 * => ある状態からの遷移は、次に追加する辺の候補を全て試し、その辺の端点二つがbに含まれない場合追加すればよい
 * => dp[2^n - 1] が答え
 */
fn main() {
    input! {
        n: usize,
    }

    let mut weight = vec![vec![0; n]; n];
    for i in 1..n {
        input! {
            d: [usize; n-i],
        }
        for (j, &d_j) in d.iter().enumerate() {
            weight[i - 1][i + j] = d_j;
            weight[i + j][i - 1] = d_j;
        }
    }

    let mut dp = vec![0; 1 << n];
    for bit in 0..((1 << n) - 1) {
        let mut v = n;
        for i in 0..n {
            if bit & (1 << i) == 0 {
                v = i;
                break;
            }
        }

        // vを含む辺を選ばないとき
        dp[bit | (1 << v)] = dp[bit | (1 << v)].max(dp[bit]);

        // vを含む辺を選ぶとき
        for pair_v in 0..n {
            if bit & (1 << pair_v) == 0 && pair_v != v {
                let next_bit = bit | (1 << v) | (1 << pair_v);
                dp[next_bit] = dp[next_bit].max(dp[bit] + weight[v][pair_v]);
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}

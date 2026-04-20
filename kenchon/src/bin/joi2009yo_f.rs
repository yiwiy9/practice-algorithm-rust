use ac_library::ModInt as Mint;
use proconio::input;

const MOD: u32 = 100000;

/// https://atcoder.jp/contests/joi2009yo/tasks/joi2009yo_f
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   一つ前に選んだ数字と現時点での合計だけ持てばいい
/// - それについて、何が分かれば答えになる？
///   N*N回処理を行った後の合計がSである場合の数の和
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   ビンゴカードではなく、N*N要素の1次元の昇順の配列を作ることを考えればいい
///   数字は昇順なので一つ前に選んだ数字だけを持てばそれまで何を選んだかは不要
///   遷移をimos法で行うことで計算量を落とす
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   dp[i][j][k] := i桁目の数字にjを選んだとき、i桁目までの数字の和がkである場合の数
///   dp[i][j][k] += dp[i][j-1][k-1]
///   dp[i+1][j+1][k+j+1] += dp[i][j][k]
///   dp[i+1][m+1][(k+m).min(s) + 1] -= dp[i][j][k]
///
/// AC: 48分
///
/// ACしたけど難しく考えすぎていた。以下のようにシンプルに考えられる
/// https://drken1215.hatenablog.com/entry/2024/09/20/010236
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   ビンゴカードではなく、1..M から N^2 個選ぶ話にして、v まで見た時点で「何個選んだか」と「総和」だけ持つ。
/// - それについて、何が分かれば答えになる？
///   1..M を見終えたとき、N^2 個選んで総和が S になる通り数が分かればよい。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   カード上の並びは昇順に一意に決まるので、配置や直前の値は持たなくてよい。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   v=1..M を順に見て、v を選ぶ/選ばないの部分和 DP をする。
fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
    }

    Mint::set_modulus(MOD);
    let mut dp = vec![vec![Mint::new(0); s + 2]; m + 2];

    dp[0][0] = Mint::new(1);
    for _ in 0..n * n {
        let mut next_dp = vec![vec![Mint::new(0); s + 2]; m + 2];
        for j in 0..m {
            for k in 0..s {
                if k + j + 1 > s {
                    break;
                }
                next_dp[j + 1][k + j + 1] += dp[j][k];
                next_dp[m + 1][(k + m).min(s) + 1] -= dp[j][k];
            }
        }

        for j in 1..=m + 1 {
            for k in 1..=s + 1 {
                let before = next_dp[j - 1][k - 1];
                next_dp[j][k] += before;
            }
        }

        dp = next_dp;
    }

    println!("{}", dp.iter().map(|dp_j| dp_j[s]).sum::<Mint>());
}

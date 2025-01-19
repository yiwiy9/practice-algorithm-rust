use proconio::input;

const INF: u128 = 1 << 60;

/// https://atcoder.jp/contests/abc389/tasks/abc389_e
/// https://atcoder.jp/contests/abc389/editorial/11933
/// i 番目の商品を k 個購入するとき k^2 * Pi 円かかるというのを以下のように言い換えることができます。
/// => i 番目の商品の値段はそれぞれ異なり、j 番目 (1-indexed) に安いものの値段は (2j−1) * Pi 円である。(∑(2i−1) = k^2)
///
/// 二分探索
/// ある整数 x が存在し、値段が x 以下の商品を全て購入し、値段が x+1 の商品を買えるだけ購入している。
///
/// 思考メモ
/// x を個数ではなく、値段として考えることがこの問題の最大のポイント
/// 個数だと、p_i で決めた k_i が 以降の p_j で決めた k_j に影響を与えるが、値段だと影響を与えない
/// => p_i ごとの値を「独立」に考えることができる
///
/// このように考えると、p_i の商品がそれぞれ別の値段で購入されるという発想が生まれる
/// あとは、値段の和が m 以下になるように p_i の商品を購入する組み合わせを考える
fn main() {
    input! {
        n: usize,
        m: u128,
        p: [u128; n],
    }

    let mut left = 0;
    let mut right = INF;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut sum = 0;
        for &p_i in &p {
            // (2j - 1) * p_i <= mid となる最大の j を求める
            let j = (mid + p_i) / (2 * p_i);
            sum += j * j * p_i; // over flow するので、 u128 で計算する
        }

        if sum <= m {
            left = mid;
        } else {
            right = mid;
        }
    }

    let mut price_sum = 0;
    let mut count_sum = 0;
    for &p_i in &p {
        let j = (left / p_i + 1) / 2;
        price_sum += j * j * p_i;
        count_sum += j;
    }

    // 値段が left + 1 の商品を買えるだけ買う
    let target = left + 1;
    for &p_i in &p {
        if price_sum + target > m {
            break;
        }

        if target > p_i && (target - p_i) % (2 * p_i) == 0 {
            price_sum += target;
            count_sum += 1;
        }
    }

    println!("{}", count_sum);
}

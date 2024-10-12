use proconio::input;
const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/adt_all_20240801_3/tasks/abc356_d
 * https://atcoder.jp/contests/adt_all_20240801_3/editorial/10126
 * 結局前回と同じ考え方をして、実装ダルなってやめてる。
 *
 * > 0 以上 N 以下の整数のうち j bit目が立っているものの個数を求めよ。
 *
 * これの高速化を考えるため、以下の性質を利用する。
 *
 * - k を非負整数とした際、 0 以上 k×2^(j+1) 未満の整数のうち、 j bit目が立っているものは k×2^j 個ある。
 *   - i の j bit目と i + 2^(j+1) の j bit目は一致する
 *   - 初めて j bit目が立つまでの場合の数をもう一度繰り返せば、j+1 bit目が立つ
 *     => j bit目が立ってから j+1 bit目が立つまでの場合の数は、j bit目が立つまでの場合の数と同じ = 2^j
 *
 * - k を非負整数、l を 2^(j+1) 未満の整数としたとき、
 *   k×2^(j+1) 以上 k×2^(j+1) + l 以下の整数のうち j bit目が立っているものの数は以下の通りである。
 *   - l が 2^j 未満のとき、0
 *   - l が 2^j 以上のとき、l - 2^j + 1 個（常に j bit目が立つ）
 *
 * そして、0~60のbitについて、それぞれのbitが立っている数を足し合わせると、答えになる。
 * => 例えば、5bit目が立っている数を求める場合、3bit目が立っている場合も重複して数えられるが、popcountであるので問題ない。
 */
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = 0;
    for i in 0..60 {
        if (m >> i) & 1 == 1 {
            ans += calc(i, n);
            ans %= MOD;
        }
    }

    println!("{}", ans);
}

fn calc(j: usize, n: usize) -> usize {
    let k = n / (1 << (j + 1));
    let l = n % (1 << (j + 1));
    k * (1 << j) + if l < (1 << j) { 0 } else { l - (1 << j) + 1 }
}

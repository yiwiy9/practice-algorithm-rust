use proconio::input;

/**
 * https://atcoder.jp/contests/abc365/tasks/abc365_e
 * https://atcoder.jp/contests/abc365/editorial/10607
 *
 * XOR を扱う問題では、各bit 毎に分けて考えることが有効である場合が多いです。
 * j=0,1,…,27 に対して、Ai の j bit 目が 1 ならば Bi =1 、0 ならば Bi =0 として長さ N の数列 B を定義します。
 * 各 j について、B に対して本問題を解き、答えに 2^j を掛けたものの総和が A に対する答えとなります。
 *
 * Bi ⊕ Bi+1 ⊕…⊕ Bj を高速に求める方法として、XOR の累積和を利用します。
 * C0 = 0, Ci = B1 ⊕…⊕ Bi として累積XORの配列 C を定めます。
 * このとき、Bi ⊕ Bi+1 ⊕…⊕ Bj = Ci−1 ⊕ Cj となります。(B1 ⊕…⊕ Bi-1 を反転させて消すイメージ)
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0_usize;
    for j in 0..28 {
        let mut b = vec![0; n];
        for i in 0..n {
            b[i] = (a[i] >> j) & 1;
        }

        let mut c = vec![0; n + 1];
        for i in 0..n {
            c[i + 1] = c[i] ^ b[i];
        }

        let mut on_cnt = 0;
        let mut off_cnt = 0;
        for i in 0..n + 1 {
            if c[i] == 1 {
                on_cnt += 1;
            } else {
                off_cnt += 1;
            }
        }

        let j_ans = (on_cnt * off_cnt) - b.iter().sum::<usize>();
        ans += j_ans * (1 << j);
    }

    println!("{}", ans);
}

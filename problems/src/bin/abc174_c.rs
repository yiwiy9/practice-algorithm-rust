use proconio::input;

/**
 * 鳩の巣原理：m個のものをｎ個の箱にどのように分配しても、必ず２個以上のものが入っている箱が少なくとも１つは存在する
 *
 * a_i = 10 * a_i-1 + 7 mod K より、a_i = a_i-1となっt時点で以降の a_i は全て同じである
 * よって、mod K において鳩の巣原理より、1 <= i <= k を調べれば事足りることがわかる
 */
fn main() {
    input! {
        k: usize,
    }

    let mut ans = -1;
    let mut cur = 0;
    for i in 1..=(k as i32) {
        cur *= 10;
        cur += 7;
        cur %= k;
        if cur == 0 {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}

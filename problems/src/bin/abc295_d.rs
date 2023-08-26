use proconio::{input, marker::Chars};

/**
 * https://atcoder.jp/contests/abc295/tasks/abc295_d
 * https://atcoder.jp/contests/abc295/editorial/6034
 *
 * 先頭から i 文字 ( 0≤i≤∣S∣ ) に x が出現した回数を 2 で割った余りを R_{i,x} とします。
 * => Ri = Rj ( i<j ) であれば S の i+1 文字目から j 文字目までを抜き出した列が嬉しい列
 *    Ri ≠ Rj ( i<j ) であれば S の i+1 文字目から j 文字目までを抜き出した列が嬉しい列でない
 */
fn main() {
    input! {
        s: Chars,
    }

    let mut r_cnt = vec![0; 1 << 10];
    let mut cur_r = 0; // 初期状態は全数字の出現回数が0
    r_cnt[cur_r] += 1;

    let mut ans = 0_usize;
    for &c in &s {
        cur_r ^= 1 << (c as i32 - '0' as i32);
        ans += r_cnt[cur_r];
        r_cnt[cur_r] += 1;
    }

    println!("{}", ans);
}
